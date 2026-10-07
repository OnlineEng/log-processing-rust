## AI Use

I feel I've fallen behind so I've used more AI in this task than I'd have liked.

Primary use was as a secondary teacher:
- Confirming I understood the required logic and correct syntax for each function
- Clarifying how ownership behaves + ordering:
    - `mut log: String` hands data in and out
    - `&mut String` lets a fn change the data with no return value
  
- Other learnings:
    - `&str` params are flexible, here it accepted `&raw_log`, `"Disk space.."` and a slice
    - Compiler errors are very readable and usually right -- made two errors and the fixes were in the messages, e.g.:
        - `string` vs `String`
        - `let` outside of function