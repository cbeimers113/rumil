#include <iostream>
#include "rumil.h"

int main(int argc, char** argv)
{
    Ast *ast = parse_file("../example/hello_world.rum");

    if (!ast)
    {
        std::cout << "Parsing failed!" << std::endl;
        return 1;
    }

    std::cout << "Parsing succeeded!" << std::endl;
    std::cout << "Ast name is " << ast_name(ast) << std::endl;
    std::cout << "Compiling..." << std::endl;
    
    // codegen
    
    free_ast(ast);

    return 0;
}
