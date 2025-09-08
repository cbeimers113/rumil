#include "cli/cli.hpp"

// Rumil CLI entry point. Packages CLI args and sends them off to invoke the requested logic.
int main(int argc, char **argv)
{
    std::vector<std::string> args(argv, argv + argc);
    return parse_args(args);
}
