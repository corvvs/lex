#include <stdlib.h>
#include <string.h>

void	*ft_memcpy(void *dst, const void *src, size_t n)
{
	unsigned char		*ud;
	const unsigned char	*us;

	if (dst == src || n == 0)
	{
		return (dst);
	}
	ud = dst;
	us = src;
	while (n--)
	{
		*ud++ = *us++;
	}
	return (dst);
}

char	*ft_strndup(const char *s1, size_t n)
{
	const size_t	len = strnlen(s1, n);
	char			*dup;

	dup = malloc(sizeof(char) * (len + 1));
	if (dup != NULL)
	{
		ft_memcpy(dup, s1, len);
		dup[len] = '\0';
	}
	return (dup);
}

void	free_strarr(void **strs) {
	size_t 	i  = 0;
	while (strs[i] != NULL)
		free(strs[i++]);
	free(strs);
}

static char const	*skip_delimiters(char const *s, char c)
{
	while (*s != '\0' && *s == c)
	{
		++s;
	}
	return (s);
}

static char const	*skip_word(char const *s, char c)
{
	while (*s != '\0' && *s != c)
	{
		++s;
	}
	return (s);
}

static char	**allocate_words(char const *s, char c)
{
	size_t		n;
	char const	*t;

	n = 0;
	while (*s != '\0')
	{
		s = skip_delimiters(s, c);
		t = skip_word(s, c);
		if (s == t)
			break ;
		s = t;
		++n;
	}
	return (malloc(sizeof(char *) * (n + 1)));
}

static char	**fill_words(char const *s, char c, char **words)
{
	size_t		i;
	char const	*t;

	i = 0;
	while (*s)
	{
		s = skip_delimiters(s, c);
		t = skip_word(s, c);
		if (s == t)
			break ;
		words[i] = strndup(s, t - s);
		if (words[i++] == NULL)
		{
			free_strarr((void**)words);
			return (NULL);
		}
		s = t;
	}
	words[i] = NULL;
	return (words);
}

char	**ft_split(char const *s, char c)
{
	char	**words;

	if (s == NULL)
	{
		return (NULL);
	}
	words = allocate_words(s, c);
	if (words == NULL)
	{
		return (NULL);
	}
	return (fill_words(s, c, words));
}
