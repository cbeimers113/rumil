#pragma once

#ifdef __cplusplus
extern "C"
{
#endif

    struct Ast;

    Ast *parse_file(const char *filepath, bool verbose);
    void free_ast(Ast *ast);

#ifdef __cplusplus
}
#endif
