#!/usr/bin/make -f

PATCH = $(wildcard *.patch               )
FILES = $(patsubst %.patch,%    ,$(PATCH))
FIXES = $(patsubst %.patch,%.fix,$(PATCH))

.PHONY: all
all:
	dos2unix $(FILES)
	$(MAKE) -f $(MAKEFILE_LIST) $(FIXES)
%.fix: %
	patch -u $< $<.patch && touch $@
# rm Core/Src/syscalls.c
