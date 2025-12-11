# Epis - Known Issues and TODOs

This document lists all known issues, TODOs, FIXMEs, and improvement areas in the Epis codebase.

## Configuration Issues

### Issue #1: Strict Type Parsing for Configs
**File:** `epis/src/config.rs:3-4`  
**Type:** FIXME  
**Description:** The current implementation doesn't parse strict types for configs. For example, a URL for `database_url` config should be parsed as a proper URL type rather than a string.  
**Reference:** https://github.com/mkermani144/epis/issues/1

### Issue #2: Support Different AI Providers
**File:** `epis/src/config.rs:19`  
**Type:** TODO  
**Description:** Currently only OpenAI is supported as an AI model provider. The system should support different providers beyond OpenAI.  
**Reference:** https://github.com/mkermani144/epis/issues/2

## Domain Model Issues

### Issue #5: Decide on Supported Languages
**File:** `epis/src/domain/models.rs:11-12`  
**Type:** TODO  
**Description:** Need to make a decision on which languages will be officially supported by Epis chatmates. Currently supports: English (En), Spanish (Es), and Turkish (Tr).  
**Reference:** https://github.com/mkermani144/epis/issues/5

## AI Agent Issues

### Issue #6: Handle Unidentified CEFR Level
**File:** `epis/src/domain/realtime_ai_agent.rs:115-116`  
**Type:** TODO  
**Description:** Need to properly handle the case when a user's CEFR level is not yet identified. Currently defaults to A1, but this should be handled more gracefully.  
**Reference:** https://github.com/mkermani144/epis/issues/6

### Issue #7: Handle Critical Operation Failures
**File:** `epis/src/domain/realtime_ai_agent.rs:203-204`  
**Type:** TODO  
**Description:** Need to handle the case where the credit spending operation fails. This is a critical operation that could lead to inconsistencies if not properly managed.  
**Reference:** https://github.com/mkermani144/epis/issues/7

## OpenAI Integration Issues

### Issue #10: Dynamic Max Tokens Configuration
**File:** `epis/src/outbound/openai.rs:161-162`  
**Type:** TODO  
**Description:** The max tokens for generation requests should be set dynamically based on data/context rather than using a hardcoded value (currently 10000).  
**Reference:** https://github.com/mkermani144/epis/issues/10

## Database/Performance Issues

### Issue #11: Batch Upsert for Learned Vocabulary
**File:** `epis/src/outbound/postgres.rs:224-226`  
**Type:** FIXME  
**Priority:** Performance  
**Description:** The current implementation upserts learned vocabulary one record at a time, which is very slow. Should implement batch upsert when sqlx supports it. This is waiting on upstream sqlx support.  
**References:**
- https://github.com/launchbadge/sqlx/issues/294
- https://github.com/mkermani144/epis/issues/11

## Summary

**Total Issues:** 7

**By Type:**
- FIXME: 2 (Issues #1, #11)
- TODO: 5 (Issues #2, #5, #6, #7, #10)

**By Category:**
- Configuration: 2 issues
- Domain Models: 1 issue
- AI Agent: 2 issues
- OpenAI Integration: 1 issue
- Database/Performance: 1 issue

**Priority Areas:**
1. Performance: Issue #11 (batch upsert)
2. Error Handling: Issue #7 (critical operation failures)
3. User Experience: Issue #6 (CEFR level handling)
4. Architecture: Issue #2 (multiple AI providers), Issue #1 (type safety)
5. Product: Issue #5 (language support decision), Issue #10 (dynamic token limits)
