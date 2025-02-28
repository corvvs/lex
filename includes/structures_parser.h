#ifndef YO_STRUCTURES_PARSER_H
# define YO_STRUCTURES_PARSER_H
# include <stdint.h>
# include <stdbool.h>

// 定義部/ルール部で扱う各種要素の種類
typedef enum e_input_item_type {
	DEF_UNKNOWN = 0,			// 不明な要素
	DEF_CODE_BLOCK,				// %{ ... %} に囲まれたコードブロック
								// ASSERT: start_line + 1 <= end_line - 1

	DEF_CODE_LINE,				// 行頭の空白で始まるコード行
								// ASSERT: start_line + 1 == end_line

	DEF_YYTEXT_TYPE,			// (%array, %pointer) yytext 型定義
								// ASSERT: start_line + 1 == end_line

	DEF_START_CONDITION,		// %s, %S による開始条件定義
								// ASSERT: start_line + 1 == end_line

	DEF_EXCLUSIVE_CONDITION,	// %x, %X による開始条件定義
								// ASSERT: start_line + 1 == end_line

	DEF_TABLE_SIZE,				// %* n によるテーブルサイズ指定
								// ASSERT: start_line + 1 == end_line

	DEF_SUBSTITUTION,			// <name> <substitution> の文字列置換定義
								// ASSERT: start_line + 1 == end_line

	DEF_RULE,					// ルール定義
								// ASSERT: start_line + 1 <= end_line - 1

} t_input_item_type;

// 各定義要素を表す構造体
typedef struct s_input_item {
    t_input_item_type	type;
	uint64_t			start_line;	// 開始行
	uint64_t			end_line;	// 終了行
									// [start_line, end_line) の行がこの定義に対応するので,
									// この情報を使って対応する部分へのアクセスを行う.
	uint64_t			re_end_pos; // 正規表現の終端位置
									// type == DEF_RULE の時のみ意味がある
    // 必要に応じて、開始条件なら対象の状態名や種別、テーブルサイズなら対象のテーブル識別子など追加情報も持たせる
} t_input_item;

// 定義部/ルール部全体を保持する構造体
typedef struct s_section_def_rules {
    uint64_t		item_cap;
    uint64_t		item_used;
    t_input_item**	items;  // 動的配列（各要素は t_input_item*）
} t_section_def_rules;

typedef struct s_items {
    uint64_t		item_cap;
    uint64_t		item_used;
    t_input_item**	items;  // 動的配列（各要素は t_input_item*）
} t_items;

typedef struct s_section_definitions {
	t_items			items;
} t_section_definitions;

typedef struct s_section_rules {
	t_items			items;
	bool			rules_emerged;
} t_section_rules;

// ユーザサブルーチン部構造体
typedef struct s_section_user_subroutines {
	uint64_t   start_line;	// 開始行
	uint64_t   end_line;	// 終了行
							// [start_line, end_line) がユーザーサブルーチン部に対応する
}	t_section_user_subroutines;

// 仮パースされた入力
typedef struct s_parsed_input {
	char**						lines;
	t_section_definitions		definitions;
	t_section_rules				rules;
	t_section_user_subroutines	user_subroutines;
}	t_parsed_input;

#endif
