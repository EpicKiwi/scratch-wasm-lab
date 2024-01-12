#include <emscripten.h>

#define NAME_LENGTH 69

char *NAMES[NAME_LENGTH] = {"Abricot", "Airelle", "Alkékenge", "Amande", "Amélanche", "Ananas", "Arbouse", "Argouse", "Asimine", "Avocat", "Banane", "Bergamote", "Orange amère", "Bleuet", "Canneberge", "Cantaloup", "Cassis", "Cerise", "Châtaigne", "Citron", "Clémentine", "Coing", "Cornouiller du Canada", "Cynorrhodon", "Datte", "Épine-vinette", "Feijoa", "Figue", "Figue de barbarie", "Fraise", "Framboise", "Grenade", "Griotte", "Groseille", "Jujube", "Kaki", "Kiwaï", "Kiwi", "Lime", "Mandarine", "Marron", "Melon", "Mûre", "Myrtille", "Nèfle", "Nèfle du Japon", "Noisette", "Noix", "Olive", "Orange", "Pamplemousse", "Pastèque", "Pêche", "Brugnon", "nectarine", "pavie", "Coqueret du Pérou", "Pistache", "Plaquebière ou chicouté", "Poire", "Pomme", "Pomélos", "Prunes", "Pruneaux", "Mirabelle", "Quetsche", "Reine-claude", "prune citron", "Raisin"};

static int currentName = 0;

EMSCRIPTEN_KEEPALIVE
void reset()
{
    currentName = 0;
}

EMSCRIPTEN_KEEPALIVE
char *nextFruit()
{
    if (currentName < NAME_LENGTH)
    {
        currentName += 1;
        return NAMES[currentName - 1];
    }
    else
    {
        return 0;
    }
}