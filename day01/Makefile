# Makefile - generic makefile template for /r/dailyprogrammer
#
# Replace TEMPLATE with the name of the target.
#
# 2017-02-10/Stephen Houser
#
DAY = $(shell basename $$PWD)
TARGET = solution
HEADERS =
SOURCES = $(TARGET).cpp

# Gnu make specific!
ifdef $(SOURCES)
# Include only .c files from $(SOURCES)
C_SOURCES = $(filter %.c,$(SOURCES))
CXX_SOURCES = $(filter %.cpp,$(SOURCES))
OBJECTS = $(C_SOURCES:.c=.o) $(CXX_SOURCES:.cpp=.o)
else
# Include all .c files in current directory
SOURCES = $(wildcard *.cpp)
HEADERS = $(wildcard *.h)
OBJECTS = $(patsubst %.cpp, %.o, $(wildcard *.cpp)) \
			$(patsubst %.c, %.o, $(wildcard *.c))
endif

# C Preprocessor flags (for c and c++ code)
CPPFLAGS = -O3 -Wall -Weffc++ -Wextra -Wconversion -Wsign-conversion -Werror -Wpedantic

# C specific flags
CC = gcc
CFLAGS = -std=c23
LFLAGS =

# C++ specific flags
CXX = g++
CXXFLAGS = -std=c++23
LXXFLAGS =

.PHONY: default all clean distclean

#default: $(TARGET)
default: test

all: default

info:
	@echo SOURCES= $(SOURCES)
	@echo HEADERS= $(HEADERS)
	@echo OBJECTS= $(OBJECTS)

# default rule for compiling c code
%.o: %.c $(HEADERS)
	$(CC) -c $(CPPFLAGS) $(CFLAGS) $< -o $@

# default rule for compiling c++ code
%.o: %.cpp $(HEADERS)
	$(CXX) -c $(CPPFLAGS) $(CXXFLAGS) $< -o $@

# https://www.gnu.org/software/make/manual/html_node/Special-Targets.html
# if make is killed or interrupted during the execution of their
# recipes, the target is not deleted.
.PRECIOUS: $(TARGET) $(OBJECTS)

$(TARGET): $(OBJECTS)
	$(CXX) $(LXXFLAGS) $^ $(LIBS) -o $@

lint: $(SOURCES)
	clang-tidy $(SOURCES)

test: $(TARGET)
	@-./$(TARGET) -v test.txt || true

input: $(TARGET)
	@-./$(TARGET) input.txt || true

summary: $(TARGET)
	@-head -1 README.md
	@-./$(TARGET) test.txt || true
	@-./$(TARGET) input.txt || true

clean:
	-rm -f *.o
	-rm -f core a.out
	-rm -f $(TARGET)
	
distclean: clean
	-rm -f $(TARGET)
