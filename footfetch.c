#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/utsname.h>
#include <sys/sysinfo.h>
#include <pwd.h>

#define MAX_LINE_LENGTH 256
#define RESET   "\033[0m"
#define BOLD    "\033[1m"
#define CYAN    "\033[36m"
#define YELLOW  "\033[33m"

typedef struct {
    char distro[MAX_LINE_LENGTH];
    char kernel[MAX_LINE_LENGTH];
    char uptime[MAX_LINE_LENGTH];
    char shell[MAX_LINE_LENGTH];
    char cpu[MAX_LINE_LENGTH];
    char gpu[MAX_LINE_LENGTH];
    char de_wm[MAX_LINE_LENGTH];
    char mem_info[MAX_LINE_LENGTH];
} FetchData;

void get_distro(char *distro) {
    FILE *fp = fopen("/etc/os-release", "r");
    if (fp) {
        char line[MAX_LINE_LENGTH];
        while (fgets(line, sizeof(line), fp)) {
            if (strncmp(line, "PRETTY_NAME=", 12) == 0) {
                char *start = strchr(line, '"');
                if (start) {
                    start++;
                    char *end = strrchr(start, '"');
                    if (end) *end = '\0';
                    strncpy(distro, start, MAX_LINE_LENGTH - 1);
                }
                break;
            }
        }
        fclose(fp);
    } else {
        strcpy(distro, "Linux");
    }
}

void get_cpu_model(char *cpu) {
    FILE *fp = fopen("/proc/cpuinfo", "r");
    if (fp) {
        char line[MAX_LINE_LENGTH];
        while (fgets(line, sizeof(line), fp)) {
            if (strncmp(line, "model name", 10) == 0) {
                char *colon = strchr(line, ':');
                if (colon) {
                    char *name = colon + 2;
                    name[strcspn(name, "\n")] = 0;
                    strncpy(cpu, name, MAX_LINE_LENGTH - 1);
                    break;
                }
            }
        }
        fclose(fp);
    }
}

void get_gpu_model(char *gpu) {
    FILE *fp = popen("lspci | grep -iE 'vga|3d' | cut -d ':' -f3 | sed 's/ (rev .*//' | sed 's/^ //'", "r");
    if (fp) {
        if (fgets(gpu, MAX_LINE_LENGTH, fp)) {
            gpu[strcspn(gpu, "\n")] = 0;
        } else {
            strcpy(gpu, "Unknown GPU");
        }
        pclose(fp);
    }
}

void get_mem_usage(char *output) {
    FILE *fp = fopen("/proc/meminfo", "r");
    unsigned long total = 0, available = 0;
    if (fp) {
        char line[MAX_LINE_LENGTH];
        while (fgets(line, sizeof(line), fp)) {
            if (sscanf(line, "MemTotal: %lu kB", &total) == 1);
            if (sscanf(line, "MemAvailable: %lu kB", &available) == 1);
        }
        fclose(fp);
        sprintf(output, "%.1f GB / %.1f GB", (float)(total - available) / 1024 / 1024, (float)total / 1024 / 1024);
    }
}

void get_de_wm(char *de_wm) {
    char *env = getenv("XDG_CURRENT_DESKTOP");
    if (!env) env = getenv("DESKTOP_SESSION");
    if (env) strncpy(de_wm, env, MAX_LINE_LENGTH - 1);
    else strcpy(de_wm, "N/A");
}

void print_colors() {
    printf(CYAN "⠀⠀⠀⠀⠈⡄⠀⠀⠀⠀⠀⠸⠀  ");
    for (int i = 0; i < 8; i++) printf("\033[4%dm  ", i);
    printf(RESET "\n");
}

int main() {
    FetchData d;
    struct utsname sys;
    struct passwd *pw = getpwuid(getuid());
    char host[MAX_LINE_LENGTH];

    uname(&sys);
    gethostname(host, sizeof(host));
    get_distro(d.distro);
    get_cpu_model(d.cpu);
    get_gpu_model(d.gpu);
    get_de_wm(d.de_wm);
    get_mem_usage(d.mem_info);

    printf("\033[2J\033[H\n");
    
    printf(CYAN "⠀⠀⠀⢀⡤⣾⠉⠑⡄⠀⠀⠀⠀  \n");
    printf(CYAN "⠀⢀⣔⠙⡄⠈⡆⠀⢀⠀⠀⠀⠀  " RESET BOLD CYAN "%s" RESET "@" BOLD CYAN "%s\n" RESET, pw->pw_name, host);
    printf(CYAN "⣀⣌⠈⡆⣗⣚⠯⠚⠘⢆⠀⠀⠀  " RESET "─────────────────────────────\n");
    printf(CYAN "⠘⡺⠁⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀  " RESET BOLD YELLOW "OS:      " RESET "%s\n", d.distro);
    printf(CYAN "⢸⠀⠀⠀⠀⢄⠀⠀⠀⠀⡎⠀⠀  " RESET BOLD YELLOW "Kernel:  " RESET "%s\n", sys.release);
    printf(CYAN "⠈⡄⠀⠀⠀⠘⠄⠀⢀⡜⠀⠀⠀  " RESET BOLD YELLOW "DE/WM:   " RESET "%s\n", d.de_wm);
    printf(CYAN "⠀⠘⡄⠀⠀⠀⠈⠠⠎⡇⠀⠀⠀  " RESET BOLD YELLOW "CPU:     " RESET "%s\n", d.cpu);
    printf(CYAN "⠀⠀⠘⡄⠀⠀⠀⠀⠀⠇⠀⠀⠀  " RESET BOLD YELLOW "GPU:     " RESET "%s\n", d.gpu);
    printf(CYAN "⠀⠀⠀⠘⡀⠀⠀⠀⠀⠘⡄⠀⠀  " RESET BOLD YELLOW "Memory:  " RESET "%s\n", d.mem_info);
    
    printf(CYAN "⠀⠀⠀⠀⢡⠀⠀⠀⠀⠀⠈⡄⠀  \n");
    print_colors();
    printf(CYAN "⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⢀⠆  \n");
    printf(CYAN "⠀⠀⠀⠀⠀⠀⠳⠄⣀⣀⠤⠊⠀  \n");

    return 0;
}