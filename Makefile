
# Michael Plekan and help from copilot
# Variables
FILES = $(wildcard *.rs)

#compile each file separately in FILES
all: $(FILES)
	for file in $(FILES); do \
		rustc $$file; \
	done

#clean up all the files in FILES without the .rs extension
clean:
	rm -f $(FILES:.rs=)