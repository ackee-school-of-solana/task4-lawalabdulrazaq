
![School of Solana](https://github.com/Ackee-Blockchain/school-of-solana/blob/master/.banner/banner.png?raw=true)

Welcome to **Task 4** of the **School of Solana Season 6**.

## 📚Task 4
In the previous task, you were introduced to the basics of Solana programming. The goal of this week's task is to further familiarize you with advanced development workflows. You will become acquainted with concepts such as PDAs, common project structures, proper storage of variable data within accounts, and more.

### Task details
This week's task is a simple Twitter App. The program can perform basic functions like **creating tweets**, **adding/removing tweet reactions** (like/dislike, you can extend it freely, but remember new reactions are not part of evaluation), and **adding/removing tweet comments**.

Here's how it works:

1. Someone wants to **create a tweet**. The tweet includes a topic and content. The topic can be up to **32 bytes** in size, and the content can be up to **500 bytes**. The topic serves as one of the seeds for a PDA (for the Tweet Account), ensuring that user can create more than one tweet.

2. When someone wants to **add a reaction** to a tweet, the program creates a new reaction account (which also has a PDA) and stores the important data in it. The seeds for the PDA are chosen to prevent more than one reaction per user on one tweet.

3. Lastly, there are **comments**, which include a field called content. This field, limited to **500 bytes**, holds the text of the comment and is also used as input for generating the comment account's PDA.

**Your task is to understand how the program works and implement all parts marked as TODO. Start within the `lib.rs`**.

### Submission Process
The source code of the on-chain program is stored within the `programs/twitter` folder, where you can also find your TODOs.

**Please do not commit changes other than those made within the `programs/twitter` folder, as it could make the evaluation process more difficult.**

### Deadline
The deadline for this task is **Wednesday, November 13th, at 23:59 UTC**.
>[!CAUTION]
>Note that we will not accept submissions after the deadline.

### Evaluation
We will evaluate your submission using the same test suite provided in this task. Therefore, the requirements for this task are to pass **100%** of the provided tests.

### Setup
For this Task you need:
- [Rust installed](https://www.rust-lang.org/tools/install)
    - Make sure to use stable version:
    ```bash
    rustup default stable
    ```
- [Solana installed](https://docs.solana.com/cli/install-solana-cli-tools)
    - Use v1.18.18
    - After you have Solana-CLI installed, you can switch between versions using:
    ```bash
    solana-install init 1.18.18
    ```

- [Anchor installed](https://www.anchor-lang.com/docs/installation)
    - Use v0.30.1
    - After you have Anchor installed, you can switch between versions using:
    ```bash
    avm use 0.30.1
    ```

### Commands
With the setup described above, you should be able to run the following commands.

- You should have **Yarn** installed as it is one of the steps during **Anchor** installation, so once you clone the repo, you should be able to run:
```
yarn install
```

- To build the project, run:
```
anchor build
```

- To test the project, run:
```
anchor test
```

### Hints and Useful Links
[Program Derived Address](https://solanacookbook.com/core-concepts/pdas.html)

[Account Context](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html)

[Account Model](https://solana.wiki/zh-cn/docs/account-model/)

[Solana Development Course](https://www.soldev.app/course)


-----

### Need help?
>[!TIP]
>If you have any questions, feel free to reach out to us on [Discord](https://discord.gg/z3JVuZyFnp).
