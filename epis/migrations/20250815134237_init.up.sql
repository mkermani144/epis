CREATE TABLE chatmate (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id TEXT NOT NULL,
    language TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    UNIQUE (user_id, language)
);
CREATE TABLE message (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chatmate_id UUID NOT NULL REFERENCES chatmate(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('user', 'ai', 'system')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
CREATE EXTENSION citext;
CREATE TABLE learned_vocab (
    chatmate_id UUID NOT NULL REFERENCES chatmate(id) ON DELETE CASCADE,
    vocab CITEXT NOT NULL,
    last_used TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    usage_count INT NOT NULL DEFAULT 1,
    streak SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY (chatmate_id, vocab)
);
