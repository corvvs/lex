#ifndef YO_LEX_H
# define YO_LEX_H
# include "libft.h"
# include "structures.h"
# include "common.h"
# include <unistd.h>
# include <stdio.h>


uint64_t	process_definition_item(char **lines, uint64_t i, t_section_definitions *defs);
uint64_t	process_rules_item(char **lines, uint64_t i, t_section_rules *rules);

bool		parse_input(t_yo *yo);


#endif
