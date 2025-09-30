# Task: Implementation Consistency & Correctness Review

## Objective

Review and fix all implementation files across multiple language ports to ensure consistency with the manual and cross-language correctness.

## Scope

Read all files from the following directories and apply necessary corrections:
- @manual/ (primary reference - DO NOT MODIFY)
- @dotnet/
- @kotlin/
- @rust/
- @go/
- @ts/
- @python/
- @r/

### 1. Manual Compliance

- **API consistency**: Verify that function names, parameters, and return types match the manual specifications
- **Algorithm correctness**: Ensure implementations follow the algorithms described in the manual
- **Mathematical accuracy**: Verify that formulas and statistical calculations match the manual
- **Behavior alignment**: Check that edge cases and special conditions are handled as documented

### 2. Cross-Language Consistency

- **Equivalent functionality**: Ensure all language implementations provide the same features
- **Consistent behavior**: Verify that all implementations produce identical results for the same inputs
- **Parameter alignment**: Check that function signatures are equivalent across languages (accounting for language idioms)
- **Error handling**: Ensure consistent error handling and validation across implementations

### 3. Implementation Quality

- **Language idioms**: Use appropriate patterns and conventions for each language
- **Code correctness**: Fix bugs, logic errors, and edge case handling
- **Type safety**: Ensure proper type usage and validation
- **Performance**: Identify obvious performance issues or inefficiencies

### 4. Language-Specific Best Practices

Each implementation should follow the idiomatic approaches and best practices of its ecosystem:

- **Naming conventions**: Use the standard naming conventions for each language (camelCase, PascalCase, snake_case, etc.)
- **Language features**: Leverage language-specific features and standard library capabilities appropriately
- **Coding conventions**: Follow established style guides and coding standards for each ecosystem
- **Ecosystem patterns**: Use common patterns and idioms that are natural to each language community
- **Type systems**: Utilize the type system capabilities (static typing, type inference, null safety, etc.) as appropriate for each language

## Constraints

- **DO NOT** modify any files in @manual/ - it is the authoritative reference
- **DO NOT** change working code unless there's a clear inconsistency or error
- **DO NOT** break existing tests without identifying and explaining the issue
- **DO NOT** add extra dependencies that we don't need
- **DO** preserve language-specific idioms and conventions
- **DO** maintain backward compatibility where possible

## Process

1. First, read the manual to understand the expected behavior and specifications
2. Scan all implementation directories to understand the current state
3. Create a systematic list of inconsistencies and errors (categorized by type and severity)
4. Fix issues incrementally, prioritizing:
   - Critical errors and bugs
   - Inconsistencies with the manual
   - Cross-language discrepancies
   - Code quality improvements
5. Report any ambiguous cases that need clarification

## Verification

- Run existing tests to ensure fixes don't break functionality
- Verify that mathematical results are consistent across implementations
- Check that API changes maintain consistency with the manual
