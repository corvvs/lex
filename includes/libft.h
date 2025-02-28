#ifndef YO_LIBFT_H
# define YO_LIBFT_H

# include <stdlib.h>

void*	ft_memcpy(void *dst, const void *src, size_t n);
size_t	ft_strnlen(const char *str, size_t n);
char*	ft_strndup(const char *s1, size_t n);
void	free_strarr(void **strs);
char**	ft_split(char const *s, char c);

#endif
