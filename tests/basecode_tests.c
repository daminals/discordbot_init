#include <criterion/criterion.h>
#include <criterion/logging.h>
#include <stdio.h>

static char *progname = "target/debug/discordbot";

Test(basecode_suite, init_equality_test) {
    // run the bot init in test output
    char *cmd = "test_output && ../target/debug/discordbot > basic_test.out && cd ..";
    // compare the output to the reference output
    char *cmp_bot = "cmp test_output/run.sh rsrc/bot";
    char *cmp_bot_txt = "cmp test_output/bot rsrc/bot";
    char *cmp_readme = "cmp test_output/README.md rsrc/readme.txt";
    char *cmp_requirements = "cmp test_output/requirements.txt rsrc/requirements.txt";

    int return_code = WEXITSTATUS(system(cmd));
    cr_assert_eq(return_code, EXIT_SUCCESS,
                 "Program exited with 0x%x instead of EXIT_SUCCESS",
		 return_code);
     
    return_code = WEXITSTATUS(system(cmp_bot));
    cr_assert_eq(return_code, EXIT_SUCCESS,
                 "Program output did not match reference output.");

    return_code = WEXITSTATUS(system(cmp_bot_txt));
    cr_assert_eq(return_code, EXIT_SUCCESS,
                 "Program output did not match reference output.");

    return_code = WEXITSTATUS(system(cmp_readme));
    cr_assert_eq(return_code, EXIT_SUCCESS,
                 "Program output did not match reference output.");

    return_code = WEXITSTATUS(system(cmp_requirements));
    cr_assert_eq(return_code, EXIT_SUCCESS,
                 "Program output did not match reference output.");

    // check if venv directory exists
    char *venv = "test_output/venv";
    FILE *fp = fopen(venv, "r");
    cr_assert_not_null(fp, "venv directory not created.");
    fclose(fp);

    // check if git was initialized
    char *git = "test_output/.git";
    fp = fopen(git, "r");
    cr_assert_not_null(fp, "git not initialized.");
    fclose(fp);

    // remove the contents of the test_output directory except for .gitkeep file
    char *rm = "rm -rf test_output/* && touch test_output/.gitkeep";
    }
