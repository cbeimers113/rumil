#ifndef RUMIL_H
#define RUMIL_H

#ifdef __cplusplus
extern "C"
{
#endif

    struct Ast;

    Ast *parse_file(const char *filepath);
    const char *ast_name(Ast *ast);
    void free_ast(Ast *ast);

#ifdef __cplusplus
}
#endif

#endif // RUMIL_H
