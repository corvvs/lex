CARGO		:=	cargo
NAME		:=  ft_lex
PROFILE_DEV	:=	dev

all:			$(NAME)

.PHONY: $(NAME)
$(NAME):
	$(CARGO) build
#	$(CARGO) build --release

.PHONY:	clean
clean:
	$(CARGO) clean --profile $(PROFILE_DEV)

.PHONY:	fclean
fclean:
	$(CARGO) clean

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
