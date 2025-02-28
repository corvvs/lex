#include "yo_lex.h"

int main(int argc, char **argv) {
	if (argc < 2) {
		dprintf(STDERR_FILENO, "Usage: %s <input_file>\n", argv[0]);
		return 1;
	}
	t_yo	yo = {};

	// [step.1] parse arguments
	yo.config.input_path = argv[1];

	// [step.2] parse input
	if (!parse_input(&yo)) {
		return 1;
	}

	// [step.3] make an automaton

	// [step.4] translate the automaton into C code
}
