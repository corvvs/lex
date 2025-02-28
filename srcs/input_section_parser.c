#include "yo_lex.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

static void add_item(t_items *items, t_input_item item_def) {
	// item をヒープに確保
	t_input_item *item = malloc(sizeof(t_input_item));
	if (item == NULL) {
		perror("malloc");
		exit(EXIT_FAILURE);
	}
	*item = item_def;
	DEBUGINFO("item: type=%d, start_line=%llu, end_line=%llu", item->type, item->start_line, item->end_line);

	// item を配列に追加
	if (items->item_used + 1 >= items->item_cap) {
		uint64_t new_cap = items->item_cap == 0 ? 8 : items->item_cap * 2;
		t_input_item **new_items = realloc(items->items, new_cap * sizeof(t_input_item *));
		if (new_items == NULL) {
			perror("realloc");
			exit(EXIT_FAILURE);
		}
		items->item_cap = new_cap;
		items->items = new_items;
	}
	items->items[items->item_used++] = item;
}

#define BEGIN_CODEBLOCK           "%{"
#define END_CODEBLOCK             "%}"
#define YYTEXT_TYPE_ARRAY         "%array"
#define YYTEXT_TYPE_POINTER       "%pointer"
#define START_CONDITION_SMALL     "%s"
#define START_CONDITION_LARGE     "%S"
#define EXCLUSIVE_CONDITION_SMALL "%x"
#define EXCLUSIVE_CONDITION_LARGE "%X"

// 定義部の1項目を処理する関数
// lines: 入力行の配列, i: 現在の行インデックス, defs: 定義部セクションへのポインタ
// 戻り値: 処理後の行インデックス（for ループの i に代入する値）
uint64_t process_definition_item(char **lines, uint64_t i, t_section_definitions *defs) {
	t_items*	items = &(defs->items);
	// コードブロック
	if (strncmp(lines[i], BEGIN_CODEBLOCK, strlen(BEGIN_CODEBLOCK)) == 0) {
		uint64_t start = i;  // コードブロックの開始行（"%{" の行）
		// コードブロックの終端を探す（行の先頭が "%}" となる行）
		while (lines[i] != NULL && strncmp(lines[i], END_CODEBLOCK, strlen(END_CODEBLOCK)) != 0) {
			i++;
		}
		if (lines[i] == NULL) {
			fprintf(stderr, "Error: コードブロックが閉じられていません (%s から始まった項目)\n", BEGIN_CODEBLOCK);
			return i;
		}
		add_item(items, (t_input_item){
			.type = DEF_CODE_BLOCK,
			.start_line = start + 1,
			.end_line = i,
		});
		return i;
	}

	// コードライン
	if (isspace((unsigned char)lines[i][0])) {
		add_item(items, (t_input_item){
			.type = DEF_CODE_LINE,
			.start_line = i,
			.end_line = i + 1
		});
		return i;
	}

	// yytext 型定義
	if (strncmp(lines[i], YYTEXT_TYPE_ARRAY, strlen(YYTEXT_TYPE_ARRAY)) == 0 ||
		strncmp(lines[i], YYTEXT_TYPE_POINTER, strlen(YYTEXT_TYPE_POINTER)) == 0) {
		add_item(items, (t_input_item){
			.type = DEF_YYTEXT_TYPE,
			.start_line = i,
			.end_line = i + 1
		});
		return i;
	}
	// 開始条件
	if (strncmp(lines[i], START_CONDITION_SMALL, strlen(START_CONDITION_SMALL)) == 0 ||
		strncmp(lines[i], START_CONDITION_LARGE, strlen(START_CONDITION_LARGE)) == 0) {
		add_item(items, (t_input_item){
			.type = DEF_START_CONDITION,
			.start_line = i,
			.end_line = i + 1
		});
		return i;
	}
	// 排他開始条件
	if (strncmp(lines[i], EXCLUSIVE_CONDITION_SMALL, strlen(EXCLUSIVE_CONDITION_SMALL)) == 0 ||
		strncmp(lines[i], EXCLUSIVE_CONDITION_LARGE, strlen(EXCLUSIVE_CONDITION_LARGE)) == 0) {
		add_item(items, (t_input_item){
			.type = DEF_EXCLUSIVE_CONDITION,
			.start_line = i,
			.end_line = i + 1
		});
		return i;
	}
	if (lines[i][0] == '%') {  // テーブルサイズ指定?
		add_item(items, (t_input_item){
			.type = DEF_TABLE_SIZE,
			.start_line = i,
			.end_line = i + 1
		});
		return i;
	}
	if (lines[i][0]) {
		for (size_t j = 1; lines[i][j]; j++) {
			if (isspace((unsigned char)lines[i][j])) {
				// 空白区切りが見つかった -> 文字列置換の定義としておく
				add_item(items, (t_input_item){
					.type = DEF_SUBSTITUTION,
					.start_line = i,
					.end_line = i + 1
				});
				return i;
			}
		}
	}

	// それ以外の場合、定義部でないとみなしてスキップ
	DEBUGWARN("Warning: 定義部でない行が出現しました: %s\n", lines[i]);
	return i;
}


#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <stdbool.h>

/**
 * 
 */
static ssize_t scan_space(const char* line) {
	if (!line) {
		return -1;
	}

	bool in_quotes = false;  // whether we're inside double-quotes
	bool escaped   = false;  // whether the current character is "escaped" by a backslash

	for (size_t i = 0; line[i] != '\0'; i++) {
		char c = line[i];

		if (!escaped) {
			if (c == '\\') {
				escaped = true;
			} else if (c == '"') {
				in_quotes = !in_quotes;
			} else if (!in_quotes && isspace((unsigned char)c)) {
				return (ssize_t)i;
			}
		} else {
			escaped = false;
		}
	}
	if (in_quotes) {
		DEBUGWARN("Warning: 文字列リテラルが閉じられていません: %s\n", line);
	}
	if (escaped) {
		DEBUGWARN("Warning: 文字列リテラルが途中で終わっています: %s\n", line);
	}

	return -1;
}

uint64_t process_rules_item(char **lines, uint64_t i, t_section_rules *rules) {
	bool rules_emerged = false;
	t_items*	items = &(rules->items);

	// コードブロック
	if (strncmp(lines[i], BEGIN_CODEBLOCK, strlen(BEGIN_CODEBLOCK)) == 0) {
		if (rules_emerged) {
			DEBUGERR("Error: ルール部の開始前にコードブロックが出現しました (%s から始まった項目)\n", BEGIN_CODEBLOCK);
			return i;
		}

		uint64_t start = i;  // コードブロックの開始行（"%{" の行）
		// コードブロックの終端を探す（行の先頭が "%}" となる行）
		while (lines[i] != NULL && strncmp(lines[i], END_CODEBLOCK, strlen(END_CODEBLOCK)) != 0) {
			i++;
		}
		if (lines[i] == NULL) {
			DEBUGERR("Error: コードブロックが閉じられていません (%s から始まった項目)\n", BEGIN_CODEBLOCK);
			return i;
		}
		add_item(items, (t_input_item){
			.type = DEF_CODE_BLOCK,
			.start_line = start + 1,
			.end_line = i,
		});
		return i;
	}

	// コードライン
	if (isspace((unsigned char)lines[i][0])) {
		if (rules_emerged) {
			DEBUGERR("Error: ルール部の開始前にコードラインが出現しました (行 %llu)\n", i);
			return i;
		}
		add_item(items, (t_input_item){
			.type = DEF_CODE_LINE,
			.start_line = i,
			.end_line = i + 1
		});
		return i;
	}

	// ルール
	// 行頭から始まる部分が正規表現の可能性がある場合, ルールとして扱う
	ssize_t space_pos = scan_space(lines[i]);
	if (space_pos >= 0) {
		// 正規表現の可能性がある
		// アクションは複数行か?
		DEBUGINFO("ルールの可能性がある: %s", lines[i]);
		char* 	open_brace = strchr(lines[i] + space_pos, '{');
		if (open_brace == NULL) {
			// 単一行アクション
			add_item(items, (t_input_item){
				.type = DEF_RULE,
				.start_line = i,
				.end_line = i + 1,
				.re_end_pos = (uint64_t)space_pos,
			});
			rules_emerged = true;
			return i;
		} else {
			// 複数行アクション
			uint64_t start_action = i;
			uint64_t end_action = 0;
			// アクション終端行を探す
			char *line = lines[i] + space_pos;
			while (line != NULL) {
				char* close_brace = strchr(line, '}');
				if (close_brace != NULL) {
					end_action = i;
					break;
				}
				i++;
				line = lines[i];
			}
			if (end_action == 0) {
				DEBUGERR("Error: 複数行アクションが閉じられていません (行 %llu から始まるルール)\n", start_action);
				return i;
			}
			add_item(items, (t_input_item){
				.type = DEF_RULE,
				.start_line = start_action,
				.end_line = end_action + 1,
				.re_end_pos = (uint64_t)space_pos,
			});
			rules_emerged = true;
			return i;
		}
	}

	// それ以外の場合、ルール部でないとみなしてスキップ
	DEBUGWARN("Warning: ルール部でない行が出現しました: %s\n", lines[i]);
	return i;
}