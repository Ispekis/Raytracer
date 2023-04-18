##
## EPITECH PROJECT, 2023
## B-FUN-400-PAR-4-1-wolfram-joseph.yu
## File description:
## Makefile
##

NAME = raytracer

all:
	cargo build
	cp target/debug/$(NAME) .

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re