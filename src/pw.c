#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>

#include <sodium.h>

#define AMAG_PW_WORDS "/usr/share/dict/words"

/* off_t fsize(const char *file) */
/* { */
/*      struct stat st; */
/*      if (stat(file, &st) == 0) */
/*           return st.st_size; */
/*      return -1; */
/* } */

char *getwords(unsigned int *count, const char *file)
{
     if (!count) {
          unsigned int count = 4;
     }

     FILE *fp;

     if ((fp = fopen(AMAG_PW_WORDS, "r")) == NULL)
          return NULL; // FIXME replace with error handling

     char *buf = malloc(4096);
}

int main()
{
     // last line is ZZZ
     /* off_t maxln = fsize(AMAG_PW_WORDS) - 1; */
     int maxln = 123115;

     unsigned int rndln = randombytes_uniform(maxln);

     printf("%d\n", rndln);

     FILE *fp;

     if ((fp = fopen(AMAG_PW_WORDS, "r")) == NULL)
          return -1; // FIXME replace with error

     return EXIT_SUCCESS;
}

