## Semana 2
# Clase #3: Configurando el entorno de trabajo
https://www.youtube.com/watch?v=Dy2OQUnPUEY

# Clase #4: Introducción a los Tokens en Solana
https://www.youtube.com/watch?v=scsryPugjHk&feature=youtu.be
Token Program
    Gestiona el token en solana
        Contiene la lógica necesaria para interactuar con tokens
    Función
        Crea una cuenta mint para representar un nuevo tipo de token
        Crear cuenta token para mantener unidades de un tipo específico de token (mint account)
        Transferir tokens de un token account a otra
            Quien recibe debe tener el token account existente
        Crear unidad de un token y agregarla a una cuenta
        Almacenar metadatos adicionales (nombre, símbolo, enlaces, etc.)

Associated Token Accounts
    derivada usando dirección del dueño y de la cuenta mint
    única y asociada a una wallet y a un mint
    cuenta por defecto

Tokens
    Todos son propiedad del Token Program
    Fungibles: 
        representan activos intercambiables y divisibles del mismo tipo y valor (USDC)
    No Fungibles (NFT)
        Representan activos individibles (por ejemplo, obras de arte)

Cuentas de tokens
    Cuenta mint
        Cada token es identificado como un Mint Account
            contador incrementado cuando se mintean tokens (creación de cuenta)
            decrementado cuando se queman (destruyen)
        Todas son propiedad del token program
    Cuenta token
        rastrea propiedad de cada unidad de token
        wallets requieren crear cuenta token para el tipo específico de token (mint o tipo de token)
            es decir, debe tener un token account que acepte el mint
