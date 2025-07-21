.PHONY: ai tmp/$(APP).ai.md
ai: tmp/$(APP).ai.md
tmp/$(APP).ai.md:
	cat README.md doc/*.md src/*.rs src/*.c* inc/*.h* src/*.ts > $@ ; touch $@
