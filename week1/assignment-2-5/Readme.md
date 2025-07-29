## Explore During Learning

- I cloned the repo of anchor and build `cargo build`
- Create a new program using `cargo init program --lib`
- Here , I gave the path of anchor directory `../anchor/lang` in `Cargo.toml` file
- Created a simple program in `src/lib.rs` file
- Imported this dependency in `Cargo.toml` file

   ```toml

   [dependencies]
   anchor-lang = { path = "../anchor/lang", features = [
    "derive",                  # Enables #[derive(Accounts)] and other Anchor macros
    "event-cpi",               # Supports emitting events in cross-program invocations (CPI)
    "interface-instructions",  # Allows defining reusable interfaces between programs
    "idl-build",               # Generates IDL file automatically during build
    "anchor-debug",            # Adds extra debug logs during execution
    "init-if-needed",          # Initializes account only if it’s not already created
    "allow-missing-optionals", # Allows missing optional fields during deserialization

  ] }

   ```
- Write program and run `cargo build` or `cargo build-sbf` for program
- Run `git diff master > week-1-assignment-2-program.patch` for creating patch file

---

## Week 1 Assignment 3 ( Solving an issue in anchor)

- https://github.com/solana-foundation/anchor/issues/3727

### Some Jargons during solving this issue

| Jargon               | Meaning (Simple Words)                                                       |
| -------------------- | ---------------------------------------------------------------------------- |
| `stream`             | A list of tokens (words/symbols from Rust code). The macro reads from this.  |
| `peek()`             | Looks ahead to see what’s coming next (like peeking in cards).               |
| `parse()`            | Reads the next item from stream and turns it into a Rust structure.          |
| `Token![:]`          | Means we are expecting a colon `:` symbol.                                   |
| `Ident::parse_any`   | Reads a name/word (like a variable name).                                    |
| `span()`             | Keeps track of where in the file something came from (helps error messages). |
| `Expr`               | A Rust expression, like `user.key()` or `"authority"`.                       |
| `parse_terminated()` | Parses a list of items with `,` in between.                                  |
| `bracketed!()`       | Reads inside square brackets like `[...]`.                                   |
| `Ident`              |It means a name in Rust, like a variable name, function name, etc.,`let my_name = "Nishant"`
| `Token![=]`          | It means we are expecting an equal sign `=` symbol.                           |
| `Token![]`           |  Rust symbol, like =, :, ,, (, ) etc.                                         |
| `ParseStream`        |This is like a token reader. It's a list of code tokens from the macro attribute. We use it to "walk" through code and read each token one by one.                                |

---


## Week 1 # Assignment #4 - Really solving the issue?

In the same issue as the last one, how would you ensure user can use any types of brackets when adding the seeds. 
So all 3 of the following should be supported


```rust

    #[derive(Accounts]) struct SomeContext<'info> {
        #[account(mut)]
        signer: Signer<'info>,
    
        #[account(init, payer = signer, size = 16, seeds =     [b"prefix", signer.key().as_ref()], bump]
        something: Account<'x, ...>,
    }

​
---------------------------------

   #[derive(Accounts]) struct SomeContext<'info> {
       #[account(mut)]
       signer: Signer<'info>,
   
       #[account(init, payer = signer, size = 16, seeds =    (b"prefix", signer.key().as_ref()), bump]
       something: Account<'x, ...>,
   }

   
---------------------------------
   #[derive(Accounts]) struct SomeContext<'info> {
       #[account(mut)]


       signer: Signer<'info>,
   
       #[account(init, payer = signer, size = 16, seeds =    {b"prefix", signer.key().as_ref()}, bump]
       something: Account<'x, ...>,
   }
```

##### For creating patching file - `git diff master..fix-issue3727  > week-1-assignment-4.patch`

---
## Week 1 Assignment #5 - Whats the error?
 https://github.com/solana-foundation/anchor/issues/3709

 - I assume ,Error is missing anchor-lang feature - `init-if-needed`
 - `anchor-lang = { path = "../anchor/lang", features = ["init-if-needed"]}`
 - <details>
   <summary>Their thought</summary>

    ```rust
      
      //It's working in agave test validtor but not working in mainnet and devnet

     #[account(
         init_if_needed,
         payer = buyer_authority,
         associated_token::authority = buyer_authority,
         associated_token::mint = nft_mint,
     )]
     pub buyer_nft_account: Account<'info, TokenAccount>,


    //So I fixed it to create an ATA in handler

    #[account(mut)]
    pub buyer_nft_account: AccountInfo<'info>,
   pub fn handle_buy_nft(
       ctx: Context<BuyNft>,
   ) -> Result<()> {
    let ata = associated_token::get_associated_token_address(
        &ctx.accounts.buyer_authority.key(),
        &ctx.accounts.nft_mint.key());
    require!(ata == ctx.accounts.buyer_nft_account.key(), CryptoVerseAIError::InvalidATA);

    if ctx.accounts.buyer_nft_account.data_len() == 0{
        //create buyer nft token account
        associated_token::create(CpiContext::new(ctx.accounts.associated_token_program.to_account_info(),
        associated_token::Create {
                payer: ctx.accounts.buyer_authority.to_account_info(),
                associated_token: ctx.accounts.buyer_nft_account.to_account_info(),
                authority: ctx.accounts.buyer_authority.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            }
        ))?;
    }}
   </details>

- I think , `init-if-needed` is by default inside in agave but not in devnet and mainnet , He should be put there.