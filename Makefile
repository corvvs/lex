SRCDIR	:=	srcs
OBJDIR	:=	objs
INCDIR	:=	includes
FILES	:=	\
			main.c\
			libft.c\
			input_section_parser.c\
			input_parser.c\

SRCS	:=	$(FILES:%.c=$(SRCDIR)/%.c)
OBJS	:=	$(FILES:%.c=$(OBJDIR)/%.o)
NAME	:=	ft_lex

CC			:=	gcc
CCOREFLAGS	=	-Wall -Wextra -Werror -O2 -I$(INCDIR)
CFLAGS		=	$(CCOREFLAGS) -g -fsanitize=address -fsanitize=undefined
RM			:=	rm -rf

all:			$(NAME)

$(OBJDIR)/%.o:	$(SRCDIR)/%.c
	@mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJDIR)/%.o:	%.c
	@mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(NAME):	$(OBJS) $(LIBFT)
	$(CC) $(CFLAGS) -o $(NAME) $(OBJS)

.PHONY:	clean
clean:
	$(RM) $(OBJDIR) $(LIBFT)

.PHONY:	fclean
fclean:			clean
	$(RM) $(NAME)

.PHONY:	re
re:				fclean all

# .PHONY:	up
# up:
# 	docker-compose up --build -d

# .PHONY:	down
# down:
# 	docker-compose down

# .PHONY:	it
# it:
# 	docker-compose exec app bash
