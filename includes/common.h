#ifndef YO_COMMON_H
# define YO_COMMON_H

// [デバッグ用出力]

# include <unistd.h>
# include <stdio.h>

# define TX_RED "\e[31m"
# define BG_RED "\e[41m"
# define TX_GRN "\e[32m"
# define TX_BLU "\e[34m"
# define TX_YLW "\e[33m"
# define TX_GRY "\e[30m"
# define TX_RST "\e[0m"

# define DEBUGSTRN(format) dprintf(STDERR_FILENO,       "%s[%s:%d %s] " format "%s", TX_GRY,   __FILE__, __LINE__, __func__, TX_RST)
# define DEBUGSTR(format) dprintf(STDERR_FILENO,        "%s[%s:%d %s] " format "%s\n", TX_GRY, __FILE__, __LINE__, __func__, TX_RST)
# define DEBUGOUT(format, ...) dprintf(STDERR_FILENO,   "%s[D] [%s:%d %s] " format "%s\n", TX_GRY, __FILE__, __LINE__, __func__, __VA_ARGS__, TX_RST)
# define DEBUGINFO(format, ...) dprintf(STDERR_FILENO,  "[I] [%s:%d %s] " format "\n",           __FILE__, __LINE__, __func__, __VA_ARGS__)
# define DEBUGWARN(format, ...) dprintf(STDERR_FILENO,  "%s[W] [%s:%d %s] " format "%s\n", TX_YLW, __FILE__, __LINE__, __func__, __VA_ARGS__, TX_RST)
# define DEBUGERR(format, ...) dprintf(STDERR_FILENO,   "%s[E] [%s:%d %s] " format "%s\n", TX_RED, __FILE__, __LINE__, __func__, __VA_ARGS__, TX_RST)
# define DEBUGSAY(format, ...) dprintf(STDERR_FILENO,   "[s] [%s:%d %s] " format "\n",           __FILE__, __LINE__, __func__, __VA_ARGS__)
# define DEBUGFATAL(format, ...) dprintf(STDERR_FILENO, "%s[F] [%s:%d %s] " format "%s\n", BG_RED, __FILE__, __LINE__, __func__, __VA_ARGS__, TX_RST)

#endif
