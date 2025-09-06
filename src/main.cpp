#include <filesystem>
#include <iostream>
#include <regex>
#include <string>

#include <CLI/CLI.hpp>
#include <rumil.h>

#include "compiler/compiler.h"
#include "interpreter/interpreter.h"

#ifndef RUMIL_VERSION
#define RUMIL_VERSION "unknown"
#endif

// Rumil entry point. Parses CLI args and invokes the requested logic.
int main(int argc, char **argv)
{
    CLI::App app{std::string("Rumil v") + RUMIL_VERSION};
    argv = app.ensure_utf8(argv);
    app.set_version_flag("--version", RUMIL_VERSION, "Print version and exit");

    // Validator lambda for .rum extension
    auto source_file_validator = [](const std::string &filename)
    {
        std::filesystem::path path(filename);
        std::regex re(R"(^[a-zA-Z0-9_.]+\.rum$)");

        if (!std::regex_match(path.filename().string(), re))
            return std::string("Source file must be (something).rum");

        return std::string();
    };

    // Validator for valid output filename (alphanumeric with internal _ or .)
    auto binary_file_validator = [](const std::string &filename)
    {
        std::regex re(R"(^[a-zA-Z0-9_.]+$)");
        std::filesystem::path path(filename);

        if (!filename.empty() && !std::regex_match(path.filename().string(), re))
            return std::string("Output binary name must only include alphanumeric chars, '_', and '.'");

        return std::string();
    };

    // User-defined args
    std::string source_file;
    std::string binary_file;
    bool verbose{false};

    // Create the "run" command
    auto cmd_run = app.add_subcommand("run", "Run a Rumil file");
    cmd_run
        ->add_option("source_file", source_file, "Source file to run")
        ->required()
        ->check(source_file_validator);
    cmd_run
        ->add_flag("-v,--verbose", verbose, "Use verbose logging");

    // Create the "build" command
    auto cmd_build = app.add_subcommand("build", "Build a Rumil program");
    cmd_build
        ->add_option("source_file", source_file, "Source file to build")
        ->required()
        ->check(source_file_validator);
    cmd_build
        ->add_option("-n,--name", binary_file, "Output binary name")
        ->default_val("")
        ->check(binary_file_validator);
    cmd_build
        ->add_flag("-v,--verbose", verbose, "Use verbose logging");

    // Parse the CLI args
    CLI11_PARSE(app, argc, argv);

    // Invoke the detected command
    bool do_run{*cmd_run};
    bool do_build{*cmd_build};

    if (do_run || do_build)
    {
        Ast *ast = parse_file(source_file.c_str(), verbose);
        int res = 1;

        if (ast)
        {
            // If no binary name was given, name it the same as the source file minus the .rum extension
            binary_file = binary_file.empty() ? source_file.substr(0, source_file.length() - 4) : binary_file;
            res = do_run ? interpret_ast(ast, verbose) : compile_ast(ast, binary_file, verbose);
        }

        free_ast(ast);
        return res;
    }

    // Print CLI help string if no valid command found
    std::cout << app.help() << "\n";
    return 0;
}
