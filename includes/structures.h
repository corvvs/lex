#ifndef YO_STRUCTURES_H
# define YO_STRUCTURES_H
# include "structures_parser.h"
# include <stdint.h>
# include <stdbool.h>

typedef struct s_option {
	// 統計情報を出力するかどうか
	// NOTE: -n 効いてる？？
	bool	is_verbose;

	// 結果を STDOUT に流すかどうか
	bool	into_stdout;

	// テーブル圧縮を行うかどうか
	bool 	do_table_compression;

}	t_option;

typedef struct t_config {
	char*		input_path;
	t_option	option;
}	t_config;


// マスター構造体
typedef struct s_yo {
	t_config		config;
	t_parsed_input	parsed_input;
}	t_yo;

#endif
