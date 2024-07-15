#  Heavy Duty Builders Rust - #Clase 5: Aprendiendo del Programa de Tokens en Solana
https://www.youtube.com/watch?v=VDk9L0e-G34

Solana Program Library SPL
spl-token --help
spl-token create-token
spl-token supply [TOKEN]
spl-token create-account [TOKEN]
spl-token accounts   
spl-token create-account [TOKEN] --owner [RECIPIENT TOKEN ACCOUNT OWNER] [--fund-recipient if recipient has no funds]
spl-token transfer [TOKEN] [AMOUNT] [RECIPIENT TOKEN ACCOUNT OWNER]
spl-token mint [TOKEN] [A

# Proyecto finalMOUNT] [CREATED-ACCOUNT]

Add Metadata
Add flag enable-metadata

spl-token create-token --enable-metadata  --program-id [ID-WITH-EXTENSIONS]

Initialize metadata:
spl-token initialize-metadata [TOKEN-ID] ["NAME"] ["SYMBOL"] ["WEB-ADDRESS"]

