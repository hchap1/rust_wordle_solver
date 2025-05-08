import pygame, random

pygame.init()
screen = pygame.display.set_mode((300, 450), vsync = True)
font = pygame.font.Font("freesansbold.ttf", 30)

running = True

with open("wordle_words.txt", "r") as file:
    words = [x.strip("\n") for x in file.readlines()]
answer = random.choice(words)

results = []
guess = ""
known_elim = []
known = []

while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

        occurences = []

        if event.type == pygame.KEYDOWN:
            unicode = event.unicode
            if unicode in "abcdefghijklmnopqrstuvwxyz" and len(guess) < 5:
                guess += unicode
            if event.key == pygame.K_BACKSPACE:
                if len(guess) > 0: guess = guess[0:len(guess) - 1]
                else: guess = ""
            if event.key == pygame.K_RETURN and len(guess) == 5:
                if guess not in words: continue
                colour = [] # 0 = gray 1 = yellow 2 = green
                for (idx, char) in enumerate(guess):
                    if char in answer:
                        if answer[idx] == char:
                            colour.append((char, 2))
                            occurences.append(char)
                            known.append(char)
                        else:
                            idxs = []
                            all = True
                            for i, c in enumerate(answer):
                                if c == char:
                                    idxs.append(i)

                            for i in idxs:
                                if guess[i] != char:
                                    all = False
                            if all or occurences.count(char) >= len(idxs):
                                colour.append((char, 0))
                                if char not in answer: known_elim.append(char)
                            else:
                                colour.append((char, 1))
                                occurences.append(char)
                    else:
                        colour.append((char, 0))
                        if char not in answer: known_elim.append(char)

                results.append(colour)
                guess = ""

    screen.fill((200, 200, 200))

    for x in range(5):
        for y in range(6):
            pygame.draw.rect(screen,
                 (180, 180, 180),
                 pygame.Rect(x * 60 + 10, y * 60 + 40, 35, 50),
                 border_radius = 10)

    for idx, result in enumerate(results):
        for i, r in enumerate(result):
            c = r[1]
            colour = (100, 100, 100)
            if c == 1: colour = (200, 200, 0)
            elif c == 2: colour = (0, 200, 0)
            screen.blit(
                font.render(r[0], True, colour),
                (20 + i * 60, 50 + idx * 60)
            )

    for (idx, char) in enumerate(guess):
        colour = (255, 255, 255)
        if char in known_elim:
            colour = (255, 200, 200)
        if char in known and answer[idx] == char:
            colour = (200, 255, 200)
        screen.blit(
            font.render(char, True, colour),
            (20 + idx * 60, 50 + len(results) * 60)
        )

    pygame.display.update()
pygame.quit()
