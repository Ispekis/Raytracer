##
## EPITECH PROJECT, 2023
## B-OOP-400-PAR-4-1-raytracer-vincent.shao
## File description:
## Makefile
##

PROGRAM_PATH = src/Raytracer

all: raytracer

PRIMITIVE = *Primitive.so
all:
	cargo build --workspace
	cp target/debug/$(NAME) .
	cp target/debug/$(PRIMITIVE) ./Plugin

clean:
	$(MAKE) clean -C $(PROGRAM_PATH)

fclean:
	$(MAKE) fclean -C $(PROGRAM_PATH)

fclean: clean
	rm -f $(NAME)
	rm -f /Plugin/$(PRIMITIVE)

re: fclean all

.PHONY: all raytracer clean fclean tests_run re
