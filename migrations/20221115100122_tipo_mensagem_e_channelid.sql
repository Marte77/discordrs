-- Add migration script here
CREATE TABLE mensagens(
    mensagem TEXT,
    idutilizador TEXT,
    nomeutilizador TEXT,
    datacriacao TEXT,
    attachments_json TEXT,
    tipo_mensagem TEXT,
    channel_id TEXT
);