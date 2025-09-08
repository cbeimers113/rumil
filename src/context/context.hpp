#include <filesystem>
#include <iostream>
#include <string>
#include <vector>

// Encapsulate metadata for a CLI invocation
struct Context
{
    // The name of the program being run
    std::string program_name;

    // The full absolute path to the source file being run
    std::string source_path;

    // The full absolute path to the directory where the program is being executed
    std::string work_dir;

    // The command line args passed to the program (minus the program name)
    std::vector<std::string> user_args;

    // Whether runtime logging is enabled
    bool verbose;

    // TODO: builtins table?

    Context(std::vector<std::string> &_args, bool _verbose)
        : user_args(_args), verbose(_verbose)
    {
        if (user_args.size() < 1)
        {
            std::cerr << "No arguments provided to invocation context\n";
            return;
        }

        // Pull out the source file path arg
        std::string source_file{user_args[0]};
        user_args.erase(user_args.begin());

        // Get the CWD
        std::filesystem::path cwd{std::filesystem::current_path()};
        work_dir = cwd.string();

        // Get the program name and source file path
        std::filesystem::path full_path{cwd / source_file};
        program_name = full_path.filename();
        source_path = full_path.string();

        // The source file should have a .rum suffix, so we'll remove it from hte program name
        if (program_name.ends_with(".rum"))
            program_name.resize(program_name.length() - 4);
    }

    std::string string()
    {
        std::string str{"Context {\n"};
        str += "    Program Name:      ";
        str += program_name;
        str += "\n    Source File Path:  ";
        str += source_path;
        str += "\n    Working Directory: ";
        str += work_dir;
        str += "\n    Verbose:           ";
        str += verbose ? "true" : "false";

        if (user_args.size() > 0)
        {
            str += "\n    User Args:         [ ";
            for (std::string arg : user_args)
            {
                str += arg;
                str += " ";
            }
            str += "]";
        }

        str += "\n}";
        return str;
    }
};