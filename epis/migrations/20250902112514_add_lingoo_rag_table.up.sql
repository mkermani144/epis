CREATE EXTENSION vector;
CREATE TABLE lingoo_rag (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    embedding vector(768) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
