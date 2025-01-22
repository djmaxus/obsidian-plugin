%%
Workflow concept
1. read the file
2. identify props not matching a group or special rearrangement words/tags
3. re-arrange tasks on file save
4. Put files to log on command
5. Consider using multiple files in a folder in future
6. move check bullet points from anywhere in the vault to inbox on command
7. Or move selections text on command
%%

<!-- comments should be deleted ? or put to references ?-->

```
Other elements not implied by the schema should be deleted
```
# GTD board %%optional? top-level heading%%

## Inbox

* [ ] **Task**
* [ ]  **Task** #next %%will be put to Next on refresh%%
* [ ]  **Task** #trash %%will be put to trash on refresh%%
*  **Task**  %%will be put to trash on refresh%%

 <!-- comments should be kept in place but skipped anywhere in the document-->
## Focus %%Only starred or (over)due today%%

* [ ] **Task 1** #project StrataTrapper #due today \
	  #tag-1 #tag-2 #contact-name \
	  Description
* [ ] ⭐️  **Task 1** #project-name\
	  #tag-1 #tag-2 #contact-name \
	  Description
* [ ] #star %%will put a star%%  **Task 1** #project-name #due 2025-04-21 \
	  #tag-1 #tag-2 #contact-name \
	  Description
## Next

* [ ] **Task 2** #project-name #due 2025-04-21 \
	  #tag-1 #tag-2 #contact-name \
	  Description

## Projects %%Maybe I don't need this section%%

### Parallel

* [ ] **Task 2** #project-name #due 2025-04-21 \
	  #tag-1 #tag-2 #contact-name \
	  Description
* [ ] **Task 2** #project-name #due 2025-04-21 \
	  #tag-1 #tag-2 #contact-name \
	  Description

### Sequential

1. [ ] **Task 2** #project-name #due 2025-04-21 \
	  #tag-1 #tag-2 #contact-name \
	  Description
2. [ ] **Task 2** #project-name #due 2025-04-21 \
	  #tag-1 #tag-2 #contact-name \
	  Description

## Waiting

* [ ]  #Dj-Maxus  **Task 1** #project-name #due @2025-04-21 \
	  #tag-1 #tag-2 #contact-name \
	  Description

## Scheduled

* [ ]  #when #2024-04-01 **Task 1** #project-name #due 2025-04-21 \
	  #tag-1 #tag-2 #contact-name \
	  Description

## Someday

* [ ]  **Task**  #due #someday %%someday criterion%%\
	  #tag-1 #tag-2 \

## References

### List

* ⭐️  Not Task 1**  %%stared on top%% \
	  #tag-1 #tag-2 #contact-name \
	  Description
## Logbook %%put tasks here manually%%

* [x] ⭐️  **Task 1** #project-name\
	  #tag-1 #tag-2 #contact-name \
	  Description

## Trash

* ⭐️  **Task 1** #project-name\
	  #tag-1 #tag-2 #contact-name \
	  Description
