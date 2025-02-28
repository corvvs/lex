#include "yo_lex.h"
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>

typedef enum e_parse_state {
	STATE_DEFINITIONS,
	STATE_RULES,
	STATE_USER_SUBROUTINES,
}	t_parse_state;


#define READ_BLOCK_SIZE 4096

static char*	read_from_fd(int fd) {
	char* ret = malloc(READ_BLOCK_SIZE + 1);
	if (ret == NULL) {
		perror("malloc");
		return NULL;
	}
	size_t ret_cap = READ_BLOCK_SIZE;
	size_t ret_used = 0;
	while (1) {
		ssize_t n = read(fd, ret + ret_used, ret_cap - ret_used);
		if (n < 0) {
			perror("read");
			free(ret);
			return NULL;
		}
		if (n == 0) {
			break;
		}
		ret_used += n;
		if (ret_used == ret_cap) {
			size_t new_cap = ret_cap * 2;
			char *realloced = realloc(ret, new_cap + 1);
			if (realloced == NULL) {
				free(ret);
				perror("malloc");
				return NULL;
			}
			ret = realloced;
			ret_cap = new_cap;
		}
	}
	DEBUGINFO("ret_used = %zu, ret_cap = %zu", ret_used, ret_cap);
	ret[ret_used] = '\0';
	return ret;
}

// read all contents of a file from `path` and return as one string.
// if `path` is NULL, read from stdin.
// return NULL if failed to read.
static char*	read_all(const char* path) {
	if (path == NULL) {
		// read from stdin
		return read_from_fd(STDIN_FILENO);
	} else {
		// read from file
		int fd = open(path, O_RDONLY);
		if (fd < 0) {
			perror("open");
			return NULL;
		}
		char*	ret = read_from_fd(fd);
		close(fd);
		return ret;
	}
}

static char**	read_lines(const char *path) {
	char*	input_content = read_all(path);
	if (input_content == NULL) {
		return NULL;
	}

	char**	lines = ft_split(input_content, '\n');
	free(input_content);
	return lines;
}

#define SECTION_DELIMITER "%%"

void	debugprint_parsed_input(const t_parsed_input* pi) {
	// 定義部
	dprintf(2, "<DEFINITIONS SECTION>\n");
	for (size_t i = 0; i < pi->definitions.items.item_used; ++i) {
		const t_input_item* item = pi->definitions.items.items[i];
		dprintf(2, "DEFINITION ITEM: type=%d, start_line=%llu, end_line=%llu\n", item->type, item->start_line, item->end_line);
	}

	// ルール部
	dprintf(2, "<RULES SECTION>\n");
	for (size_t i = 0; i < pi->rules.items.item_used; ++i) {
		const t_input_item* item = pi->rules.items.items[i];
		dprintf(2, "RULE ITEM: type=%d, start_line=%llu, end_line=%llu\n", item->type, item->start_line, item->end_line);
		if (item->type == DEF_RULE) {
			const char* start_line = pi->lines[item->start_line];
			dprintf(2, "  RE: /%.*s/\n", (int)item->re_end_pos, start_line);
			for (size_t k = item->start_line; k < item->end_line; ++k) {
				const char *line = k == item->start_line ? start_line + item->re_end_pos : pi->lines[k];
				dprintf(2, "  ACTION: %s\n", line);
			}
		}
	}

	// ユーザーサブルーチン部
	if (pi->user_subroutines.start_line == 0) {
		dprintf(2, "<USER_SUBROUTINE SECTION> (not found)\n");
		return;
	}
	dprintf(2, "<USER_SUBROUTINE SECTION>\n");
	for (size_t i = pi->user_subroutines.start_line; i < pi->user_subroutines.end_line; ++i) {
		dprintf(2, "%s\n", pi->lines[i]);
	}
}

bool	parse_input(t_yo *yo) {
	char*	input_path = yo->config.input_path;
	char**	lines = read_lines(input_path);
	if (lines == NULL) {
		return false;
	}

	for (size_t i = 0; lines[i] != NULL; ++i) {
		DEBUGINFO("lines[%zu] = %s", i, lines[i]);
	}

	t_parsed_input*	pi = &(yo->parsed_input);
	pi->lines = lines;
	t_parse_state	state = STATE_DEFINITIONS;
	size_t			i;
	for (i = 0; lines[i] != NULL; ++i) {
		const char* line = lines[i];
		switch (state) {
			case STATE_DEFINITIONS: {
				if (strcmp(line, SECTION_DELIMITER) == 0) {
					// ルール部に飛ぶ
					state = STATE_RULES;
					DEBUGINFO("state -> STATE_RULES: %zu", i);
					break;
				}
				i = process_definition_item(lines, i, &(pi->definitions));
				DEBUGINFO("i = %zu", i);
				break;
			}
			case STATE_RULES: {
				if (strcmp(line, SECTION_DELIMITER) == 0) {
					// ユーザーサブルーチン部に飛ぶ
					state = STATE_USER_SUBROUTINES;
					DEBUGINFO("state -> STATE_USER_SUBROUTINES: %zu", i);
					break;
				}

				i = process_rules_item(lines, i, &(pi->rules));
				DEBUGINFO("i = %zu", i);
				break;
			}
			case STATE_USER_SUBROUTINES: {
				if (pi->user_subroutines.start_line == 0) {
					pi->user_subroutines.start_line = i;
				}

				if (strcmp(line, SECTION_DELIMITER) == 0) {
					// ここにこれがあるのはおかしい
					DEBUGERR("found %s in USER-SUBROUTINE SECTION", SECTION_DELIMITER);
					break;
				}

				break;
			}
			default: {
				DEBUGFATAL("unexpected state: %d", state);
				free_strarr((void**)lines);
				return false;
			}
		}
	}
	if (pi->user_subroutines.start_line > 0) {
		pi->user_subroutines.end_line = i;
	}

	// 仮パース入力のデバッグ出力
	debugprint_parsed_input(pi);
	return true;
}
