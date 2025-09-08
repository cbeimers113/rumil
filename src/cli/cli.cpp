#include <functional>
#include <iostream>

#include "cli.hpp"
#include "../context/context.hpp"

// Command callbacks
int cmd_help(std::vector<std::string> &);
int cmd_version(std::vector<std::string> &);
int cmd_run(std::vector<std::string> &);
int cmd_debug(std::vector<std::string> &);
int cmd_build(std::vector<std::string> &);

// A description of a CLI command
struct Command
{
    std::string identifier;
    std::string description;
    std::function<int(std::vector<std::string> &)> callback;
    bool requires_src;

    Command(
        const std::string &id,
        const std::string &desc,
        std::function<int(std::vector<std::string> &)> cb,
        bool req_src)
        : identifier(id), description(desc), callback(cb), requires_src(req_src) {}
};

// Register the commands
std::vector<Command> commands{
    {"help", "Prints CLI help to the command line", cmd_help, false},
    {"version", "Prints the current Rumil version to the command line", cmd_version, false},
    {"run", "Executes the given source code", cmd_run, true},
    {"debug", "Executes the given source code with runtime logs", cmd_debug, true},
    {"build", "Builds the given source code into an executable binary", cmd_build, true}};

// Handle command line arguments to Rumil
int parse_args(std::vector<std::string> &args)
{
    // Handle no args provided
    if (args.size() == 1)
    {
        std::cout << "Please enter a command or run \"rumil help\" for more information\n";
        return 1;
    }

    // Grab out the cmd arg and source file arg (if it's there)
    std::string cmd{args[1]};
    std::string src{args.size() > 2 ? args[2] : ""};

    // Find the matching command
    bool shorthand{cmd.size() == 1};
    for (Command command : commands)
    {
        if (
            // Match shorthand
            shorthand && cmd[0] == command.identifier[0] ||
            // Match full command
            !shorthand && cmd == command.identifier)
        {
            // Check if we've supplied the source file arg if we need it
            if (command.requires_src && !src.ends_with(".rum"))
            {
                std::cout << "Please provide a Rumil source file for this command\n";
                return 1;
            }

            // Invoke the callback
            args.erase(args.begin());
            args.erase(args.begin());
            return command.callback(args);
        }
    }

    // Unrecognized command
    std::cout << "Unrecognized command: " << cmd << "\nPlease run \"rumil help\" for a list of valid commands\n";
    return 1;
}

// Create and print a help string for all the commands
int cmd_help(std::vector<std::string> &)
{
    std::string msg{"Rumil CLI Manual\n\n"};

    for (Command command : commands)
    {
        // Right pad command ID to 10 chars with spaces so descriptions are aligned
        msg += command.identifier;
        for (int i = 0; i < 10 - command.identifier.size(); i++)
            msg += " ";
        msg += command.description;

        msg += "\n          Usage:  rumil ";
        msg += command.identifier;
        msg += "|";
        msg += command.identifier[0];

        if (command.requires_src)
            msg += " source_file.rum";

        msg += "\n\n";
    }

    std::cout << msg;
    return 0;
}

// Print the version number of this build of the Rumil binary
int cmd_version(std::vector<std::string> &)
{
    std::cout << "Rumil v" << RUMIL_VERSION << "\n";
    return 0;
}

// Execute the code in the provided source file
int cmd_run(std::vector<std::string> &args)
{
    Context ctx{args, false};
    std::cout << ctx.string() << "\n";
    return 0;
}

// Execute the code in the provided source file with runtime logging
int cmd_debug(std::vector<std::string> &args)
{
    Context ctx{args, true};
    std::cout << ctx.string() << "\n";
    return 0;
}

// Compile the code in the provided source file into a binary
int cmd_build(std::vector<std::string> &)
{
    std::cout << "Unimplemented\n";
    return 1;
}
