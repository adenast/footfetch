#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/utsname.h>
#include <sys/sysinfo.h>
#include <pwd.h>

#define MAX_LINE_LENGTH 256
#define RESET "\033[0m"
#define BOLD "\033[1m"
#define CYAN "\033[36m"
#define GREEN "\033[32m"
#define YELLOW "\033[33m"
#define BLUE "\033[34m"
#define MAGENTA "\033[35m"

char* get_distro() {
    FILE *fp;
    static char distro[MAX_LINE_LENGTH] = "Unknown";
    char line[MAX_LINE_LENGTH];
    
    fp = fopen("/etc/os-release", "r");
    if (fp != NULL) {
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
        return distro;
    }
    
    fp = fopen("/etc/issue", "r");
    if (fp != NULL) {
        if (fgets(line, sizeof(line), fp)) {
            char *pos = strstr(line, " \\");
            if (pos) *pos = '\0';
            strncpy(distro, line, MAX_LINE_LENGTH - 1);
            distro[strcspn(distro, "\n")] = 0;
        }
        fclose(fp);
    }
    
    return distro;
}

char* get_shell() {
    static char shell[MAX_LINE_LENGTH];
    char *shell_env = getenv("SHELL");
    if (shell_env) {
        char *basename = strrchr(shell_env, '/');
        if (basename) {
            strncpy(shell, basename + 1, MAX_LINE_LENGTH - 1);
        } else {
            strncpy(shell, shell_env, MAX_LINE_LENGTH - 1);
        }
    } else {
        strcpy(shell, "Unknown");
    }
    return shell;
}

void format_memory(unsigned long mem_kb, char *output) {
    double mem_mb = mem_kb / 1024.0;
    double mem_gb = mem_mb / 1024.0;
    
    if (mem_gb >= 1.0) {
        sprintf(output, "%.1f GB", mem_gb);
    } else {
        sprintf(output, "%.0f MB", mem_mb);
    }
}

char* get_uptime() {
    static char uptime_str[MAX_LINE_LENGTH];
    struct sysinfo info;
    
    if (sysinfo(&info) == 0) {
        long uptime_seconds = info.uptime;
        int days = uptime_seconds / 86400;
        int hours = (uptime_seconds % 86400) / 3600;
        int minutes = (uptime_seconds % 3600) / 60;
        
        if (days > 0) {
            sprintf(uptime_str, "%dd %dh %dm", days, hours, minutes);
        } else if (hours > 0) {
            sprintf(uptime_str, "%dh %dm", hours, minutes);
        } else {
            sprintf(uptime_str, "%dm", minutes);
        }
    } else {
        strcpy(uptime_str, "Unknown");
    }
    
    return uptime_str;
}

void print_combined_output() {
    struct utsname sys_info;
    struct sysinfo mem_info;
    struct passwd *pw;
    char hostname[MAX_LINE_LENGTH];
    char total_mem[64], used_mem[64];
    
    uname(&sys_info);
    sysinfo(&mem_info);
    gethostname(hostname, sizeof(hostname));
    pw = getpwuid(getuid());
    
    unsigned long total_mem_kb = mem_info.totalram / 1024;
    unsigned long free_mem_kb = mem_info.freeram / 1024;
    unsigned long used_mem_kb = total_mem_kb - free_mem_kb;
    
    format_memory(total_mem_kb, total_mem);
    format_memory(used_mem_kb, used_mem);
    
    printf("\n");
    
    printf(CYAN "                   " RESET BOLD CYAN "%s" RESET "@" BOLD CYAN "%s\n" RESET, pw->pw_name, hostname);
    printf(CYAN "      Ooooo        " RESET "─────────────────────────────\n");
    printf(CYAN "      (   )        " RESET BOLD YELLOW "OS:       " RESET "%s\n", get_distro());
    printf(CYAN "       )  (        " RESET BOLD YELLOW "Kernel:   " RESET "%s\n", sys_info.release);
    printf(CYAN "       (__)        " RESET BOLD YELLOW "Uptime:   " RESET "%s\n", get_uptime());
    printf(CYAN "                   " RESET BOLD YELLOW "Shell:    " RESET "%s\n", get_shell());
    printf(CYAN "                   " RESET BOLD YELLOW "CPU:      " RESET "%s\n", sys_info.machine);
    printf(CYAN "                   " RESET BOLD YELLOW "Memory:   " RESET "%s / %s\n", used_mem, total_mem);

    printf("\n");
}

int main() {
    printf("\033[2J\033[H");

    print_combined_output();
    
    return 0;
}