#include <filesystem>
#include <iostream>
#include <string>
#include <vector>

#include <rumil.h>

// Types of invocations
enum class ContextType
{
    RUN,
    DEBUG,
    BUILD,
};

// Encapsulate metadata for a CLI invocation
struct Context
{
    // What action is happening
    ContextType context_type;

    // The name of the program being run
    std::string program_name;

    // The full absolute path to the source file being run
    std::string source_path;

    // The full absolute path to the directory where the program is being executed
    std::string work_dir;

    // The command line args passed to the program (minus the program name)
    std::vector<std::string> user_args;

    // The AST produced from the code
    Ast *ast;

    // TODO: builtins table?

    // Create an invocation context with the given parameters
    Context(ContextType _context_type, std::vector<std::string> &_args)
        : context_type(_context_type), user_args(_args)
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

        // The source file should have a .rum suffix, so we'll remove it from the program name
        if (program_name.ends_with(".rum"))
            program_name.resize(program_name.length() - 4);
    }

    // Clean up the AST when the Context is done
    ~Context()
    {
        free_ast(ast);
    }

    // Try to parse the source code into an Ast. Any errors that can arise will be emitted to stderr
    // by the parser library and we'll get a null pointer. FLushes parser logs to stdout before returning
    int parse()
    {
        ast = parse_file(source_path.c_str(), context_type == ContextType::DEBUG);
        std::cout.flush();

        if (ast)
            return 0;
        return 1;
    }

    // Create and return a string representation of an invocation context
    std::string string()
    {
        // Map the context type to a string name
        std::string type_str{"Unknown"};
        switch (context_type)
        {
        case ContextType::RUN:
            type_str = "Run";
            break;
        case ContextType::DEBUG:
            type_str = "Debug";
            break;
        case ContextType::BUILD:
            type_str = "Build";
            break;
        }

        // Build out the string representation
        std::string str{type_str};
        str += " Context {\n";
        str += "    Program Name:      ";
        str += program_name;
        str += "\n    Source File Path:  ";
        str += source_path;
        str += "\n    Working Directory: ";
        str += work_dir;

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