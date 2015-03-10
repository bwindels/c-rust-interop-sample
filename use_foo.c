#include <stdio.h>

struct TokenCollection;

TokenCollection* token_collection_create(const char*);
uint64_t token_collection_len(TokenCollection*);
void token_collection_destroy(TokenCollection*);

int main() {
	TokenCollection* col = token_collection_create("this is a sentence");
	printf("%d tokens", token_collection_len(col));
	token_collection_destroy(col);
	return 0;
}