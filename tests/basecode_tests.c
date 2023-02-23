#include <criterion/criterion.h>
#include <criterion/logging.h>
#include <stdio.h>

static char *progname = "target/debug/discordbot";

Test(basecode_suite, readme_equality) {
    // char *cmp_bot_txt = "cmp test_output/bot rsrc/bot";
    char *cmp_readme = "cmp test_output/README.md rsrc/readme.txt";

    int return_code = WEXITSTATUS(system(cmp_readme));
    cr_assert_eq(return_code, EXIT_SUCCESS,
                 "Program output did not match reference output.");
    }

Test(basecode_suite, runsh_equality) {
    char *cmp_bot = "cmp test_output/run.sh rsrc/bot";
    int return_code = WEXITSTATUS(system(cmp_bot));
    cr_assert_eq(return_code, EXIT_SUCCESS,
                 "Program output did not match reference output.");
}

Test(basecode_suite, requirements_equality) {
    char *cmp_requirements = "cmp test_output/requirements.txt rsrc/requirements.txt";
    int return_code = WEXITSTATUS(system(cmp_requirements));
    cr_assert_eq(return_code, EXIT_SUCCESS,
                 "Program output did not match reference output.");

}

Test(basecode_suite, venv_exists) {
    // check if venv directory exists
    char *venv = "test_output/venv";
    FILE *fp = fopen(venv, "r");
    cr_assert_not_null(fp, "venv directory not created.");
    fclose(fp);
    // char *cmd = "source test_output/venv/bin/activate";
    // int return_code = WEXITSTATUS(system(cmd));
    // cr_assert_eq(return_code, EXIT_SUCCESS,
    //              "venv not activated.");
}



Test(basecode_suite, git_exists) {
    // check if git was initialized
    char *git = "test_output/.git";
    FILE* fp = fopen(git, "r");
    cr_assert_not_null(fp, "git not initialized.");
    fclose(fp);
}
