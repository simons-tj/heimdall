# entities

User { id }
Subscription { id }
Content { id }
Asset { id, type }
Folder { id }


# connections

User -[member_of { role }]-> Subcription

User -[has_access { role }]-> Content
Subscription -[has_access { role }]-> Content

User -[has_access { role }]-> Folder
Folder -[contains]-> Content

Content -[contains]-> Asset

User -[owns]-> Asset
Subscription -[owns]-> Asset


# enums
## roles
0 - Owner
1 - Manager
2 - Editor

## types
image
audio
video
file
