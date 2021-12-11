#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include <sys/types.h>
#include <time.h>
#include <netdb.h>

#include <iostream>
#include <string>

#include <poll.h>

extern "C" int accept_(int32_t a, sockaddr* b, socklen_t* c) { return accept(a, b, c); }

extern "C" int bind_(uint32_t a, sockaddr* b, size_t c) { return bind(a, b, c); }

extern "C" int connect_(uint32_t a, sockaddr* b, size_t c) { return connect(a, b, c); }

extern "C" int listen_(uint32_t a, uint64_t b) { return listen(a, b); }


extern "C" int recv_(uint32_t a, char* b, uint64_t c, uint64_t d) { return recv(a, b, c, d); }


extern "C" int send_(uint32_t a, char* b, uint64_t c, uint64_t d) { return send(a, b, c, d); }


extern "C" int shutdown_(uint32_t a, uint64_t b) { return shutdown(a, b); }


extern "C" int socket_(uint64_t a, uint64_t b, uint64_t c) { return socket(a, b, c); }



extern "C" int setsockopt_(uint32_t a, uint64_t b, uint64_t c, char* d, uint64_t e) { return setsockopt(a, b, c, d, e); }


extern "C" void do_fd_set(fd_set* a, int b, size_t c)
{
	if (0 == c) FD_ZERO(a);
	if (0 == b) return;
	FD_SET(b, a);
}

extern "C" int poll_(struct pollfd* a, int b, int c) { return poll(a, b, c); }

extern "C" int select_(uint64_t a, fd_set* b, fd_set* c, fd_set* d, struct timeval* e) { return select(a, b, c, d, e); }


#include <assert.h>
extern "C" int dup_(int fd)
{
    return dup(fd);
}

extern "C" int close_(int a) { return close(a); }

