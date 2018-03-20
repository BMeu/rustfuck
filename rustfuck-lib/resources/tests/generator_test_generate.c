{
    (*ptr)++;
    ptr++;
    ptr--;
    while (*ptr) {
        *ptr = getchar();
        (*ptr)--;
    }
    putchar(*ptr);
}
