.PHONY: ai tmp/$(APP).ai.md
ai: tmp/$(APP).ai.md
tmp/$(APP).ai.md:
	cat doc/*.md src/*.c* inc/*.h* src/*.ts > $@ ; touch $@
