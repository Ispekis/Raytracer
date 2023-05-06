##
## EPITECH PROJECT, 2023
## B-OOP-400-PAR-4-1-raytracer-vincent.shao
## File description:
## Makefile
##

PROGRAM_PATH = src/Raytracer

all: raytracer

raytracer:
	$(MAKE) -C $(PROGRAM_PATH)

clean:
	$(MAKE) clean -C $(PROGRAM_PATH)

fclean:
	$(MAKE) fclean -C $(PROGRAM_PATH)

tests_run:
	echo tests

re: fclean all

.PHONY: all raytracer clean fclean tests_run re