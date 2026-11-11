# Obsidian Design System - Analyse pour Note Craft

> **Mission**: Analyse du design system d'Obsidian pour implémentation dans Note Craft
> **Source**: [obsidianmd/obsidian-releases](https://github.com/obsidianmd/obsidian-releases) - Version 1.15.0
> **Thème**: Sombre par défaut
> **Direction artistique**: Inspiré d'Obsidian

---

## Table des matières

1. [Palette de Couleurs](#1-palette-de-couleurs)
2. [Typographie](#2-typographie)
3. [Layout](#3-layout)
4. [Composants UI](#4-composants-ui)
5. [Animations et Interactions](#5-animations-et-interactions)
6. [Graphe de Connaissances](#6-graphe-de-connaissances)
7. [Recommandations pour Note Craft](#7-recommandations-pour-note-craft)

---

## 1. Palette de Couleurs

### 1.1 Couleurs de Base (Thème Sombre)

| Catégorie | Couleur | Code HEX | Usage |
|----------|---------|----------|-------|
| **Background Primary** | mono-0 | `#1C1C1C` | Fond principal de l'application |
| **Background Primary Alt** | mono-10 | `#232323` | Fond alternatif (ex: hover léger) |
| **Background Secondary** | mono-20 | `#282828` | Fond secondaire (panneaux latéraux) |
| **Background Tertiary** | mono-25 | `#2e2e2e` | Fond tertiaire |
| **Background Quaternary** | mono-30 | `#333333` | Fond quaternaire / bordures |
| **Background Quinary** | mono-35 | `#3f3f3f` | Bordures principales |
| **Background Light** | mono-40 | `#555555` | Éléments plus clairs |
| **Text Primary** | mono-100 | `#dadada` | Texte principal |
| **Text Secondary** | mono-70 | `#999999` | Texte secondaire |
| **Text Muted** | mono-60 | `#666666` | Texte atténué |
| **Text Faint** | mono-50 | `#5c5c5c` | Texte très atténué |

### 1.2 Échelle de Couleurs Base (Thème Sombre)

```
color-base-00: #1C1C1C  (Noir profond)
color-base-05: #212121
color-base-10: #232323
color-base-20: #282828
color-base-25: #2e2e2e
color-base-30: #333333
color-base-35: #3f3f3f  (Bordures)
color-base-40: #555555
color-base-50: #666666
color-base-60: #999999
color-base-70: #b3b3b3
color-base-100: #dadada (Texte clair)
```

### 1.3 Couleurs d'Accentuation

| Couleur | Code HEX | HSL | Usage |
|---------|----------|-----|-------|
| **Accent Primary** | `#027aff` | `hsl(258, 88%, 66%)` | Liens, éléments interactifs |
| **Accent Hover** | `#80ebff` | `hsl(255, 90%, 72%)` | Survol des éléments accentués |
| **Accent Active** | `#aaf2ff` | - | État actif des éléments accentués |
| **Focus Ring** | `#0df` | - | Contour de focus |

### 1.4 Couleurs de Syntaxe (Code)

| Élément | Couleur | Code HEX |
|---------|---------|----------|
| Texte normal | text-normal | `#dadada` |
| Commentaires | text-faint | `#666666` |
| **Fonctions** | color-yellow | `#e0de71` |
| **Mots-clés** | color-pink | `#fa99cd` |
| **Strings** | color-green | `#44cf6e` |
| **Balises** | color-red | `#fb464c` |
| **Opérateurs** | color-red | `#fb464c` |
| **Propriétés** | color-cyan | `#53dfdd` |
| **Ponctuation** | text-muted | `#999999` |
| **Variables importantes** | color-orange | `#e9973f` |
| **Valeurs** | color-purple | `#a882ff` |

### 1.5 Couleurs des Callouts

| Type | Couleur | Code HEX |
|------|---------|----------|
| Bug | color-red | `#fb464c` |
| Default | color-blue | `#027aff` |
| Error | color-red | `#fb464c` |
| Example | color-purple | `#a882ff` |
| Fail | color-red | `#fb464c` |
| Important | color-cyan | `#53dfdd` |
| Info | color-blue | `#027aff` |
| Question | color-orange | `#e9973f` |
| Success | color-green | `#44cf6e` |
| Summary | color-cyan | `#53dfdd` |
| Tip | color-cyan | `#53dfdd` |
| Todo | color-blue | `#027aff` |
| Warning | color-orange | `#e9973f` |
| Quote | - | `#9e9e9e` |

### 1.6 Couleurs Brand

| Couleur | Code HEX | Usage |
|---------|----------|-------|
| Rouge | `#fb464c` | Erreurs, callouts d'erreur |
| Orange | `#e9973f` | Avertissements, callouts question/warning |
| Jaune | `#e0de71` | Fonctions, accents |
| Vert | `#44cf6e` | Succès, strings, callouts success |
| Cyan | `#53dfdd` | Important, propriétés, callouts |
| Bleu | `#027aff` | Accent principal, liens, info |
| Violet | `#a882ff` | Exemple, valeurs, callouts |
| Rose | `#fa99cd` | Mots-clés |

### 1.7 Couleurs des Messages

| Type | Fond | Texte | Icône |
|------|------|-------|------|
| Message bar (default) | `#5a3100` | `#fbfbfe` | `#e49c49` |
| Border | - | - | `rgba(255, 255, 255, 0.08)` |

---

## 2. Typographie

### 2.1 Families de Polices

#### Police par défaut (Interface)
```
font-default: ui-sans-serif, -apple-system, BlinkMacSystemFont, system-ui, "Segoe UI", 
              "Google Sans Flex", Roboto, "Inter Variable", "Inter", 
              "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", sans-serif
```

#### Police Monospace (Code)
```
font-monospace-default: ui-monospace, SFMono-Regular, "Cascadia Mono", 
                      "Roboto Mono", "DejaVu Sans Mono", "Liberation Mono", 
                      Menlo, Monaco, "Consolas", "Source Code Pro", monospace
```

#### **Recommandation Note Craft: JetBrains Mono**
```
font-monospace: "JetBrains Mono", "Cascadia Mono", "Fira Code", 
                "Roboto Mono", monospace
```
> **Note**: Obsidian permet de configurer manuellement JetBrains Mono via les paramètres. Pour Note Craft, nous recommandons d'inclure JetBrains Mono comme police principale pour le code.

### 2.2 Tailles de Police

| Nom | Valeur | Usage |
|-----|--------|-------|
| `font-text-size` | `16px` | Taille de texte par défaut |
| `font-preferred-size` | `16px` | Taille préférée (variable) |
| `font-smallest` | `0.8em` | Texte très petit |
| `font-smaller` | `0.875em` | Texte petit |
| `font-small` | `0.933em` | Texte légèrement petit |
| `font-ui-smaller` | `12px` | UI - Très petit |
| `font-ui-small` | `13px` | UI - Petit (navigation, boutons) |
| `font-ui-medium` | `15px` | UI - Moyen (éditeur par défaut) |
| `font-ui-large` | `20px` | UI - Grand (titres) |
| `code-size` | `0.875em` | Code inline et blocs |
| `sidebar-markdown-font-size` | `calc(var(--font-preferred-size) * 0.9)` | Markdown dans la sidebar |

### 2.3 Poids de Police

| Nom | Valeur | Usage |
|-----|--------|-------|
| `font-thin` | `100` | Ultra-léger |
| `font-extralight` | `200` | Très léger |
| `font-light` | `300` | Léger |
| `font-normal` | `400` | Normal (par défaut) |
| `font-medium` | `500` | Moyen |
| `font-semibold` | `600` | Semi-gras |
| `font-bold` | `700` | Gras |
| `font-extrabold` | `800` | Très gras |
| `font-black` | `900` | Ultra-gras |

### 2.4 Titres (Headings)

| Niveau | Taille | Poids | Interligne | Letter Spacing | Couleur |
|--------|--------|-------|------------|----------------|---------|
| H1 | `1.618em` | `700` | `1.2` | `-0.015em` | `inherit` |
| H2 | `1.462em` | `600` | `1.2` | `-0.011em` | `inherit` |
| H3 | `1.318em` | `600` | `1.3` | `-0.008em` | `inherit` |
| H4 | `1.188em` | `600` | `1.4` | `-0.005em` | `inherit` |
| H5 | `1.076em` | `600` | `1.5` | `-0.002em` | `inherit` |
| H6 | `1em` | `600` | `1.5` | `0em` | `inherit` |

> **Note**: Avec `font-variation-settings` supporté, les poids sont légèrement ajustés (H1: 700, H2: 680, H3: 660, H4: 640, H5: 620, H6: 600)

### 2.5 Style de Texte

| Propriété | Valeur |
|-----------|--------|
| `line-height-normal` | `1.5` |
| `letter-spacing` | `-0.01px` (par défaut) |
| `code-white-space` | `pre-wrap` |
| `italic-color` | `inherit` |
| `italic-weight` | `inherit` |
| `bold-color` | `inherit` |
| `bold-weight` | `calc(var(--font-weight) + 200)` |

---

## 3. Layout

### 3.1 Header (Barre de titre)

| Propriété | Valeur |
|-----------|--------|
| `header-height` | `40px` |
| `bases-header-padding-start` | `2px` |
| `bases-header-padding-end` | `2px` |
| `bases-header-border-width` | `0 0 1px 0` |
| `titlebar-background` | `#1C1C1C` |
| `titlebar-background-focused` | `#1C1C1C` |
| `titlebar-border-width` | `0px` |
| `titlebar-border-color` | `#3f3f3f` |
| `titlebar-text-color` | `#999999` |
| `titlebar-text-color-focused` | `#dadada` |
| `titlebar-text-weight` | `700` (bold) |

### 3.2 Sidebars (Panneaux Latéraux)

#### Dimensions

| Propriété | Valeur |
|-----------|--------|
| `sidebar-width` | `140px` (largeur par défaut) |
| `sidebar-left-toggle-inner-width` (fermé) | `8.33%` |
| `sidebar-left-toggle-inner-width` (ouvert) | `24%` |
| `sidebar-right-toggle-inner-width` (fermé) | `8.33%` |
| `sidebar-right-toggle-inner-width` (ouvert) | `24%` |

#### Styles

| Propriété | Valeur |
|-----------|--------|
| `sidebar-tab-container-background` | `var(--titlebar-background)` = `#1C1C1C` |
| `sidebar-tab-text-display` | `none` |
| `mobile-sidebar-background` | `#1C1C1C` |
| `workspace-background-translucent` | `color-mix(in oklch, white 60%, transparent)` |

#### Comportement

- **Redimensionnable**: Oui, par l'utilisateur
- **Collapsible**: Oui, avec animation fluide
- **Position**: Gauche et droite
- **Sur mobile**: Masquable avec bouton toggle
- **Responsive**: S'adapte à la taille de l'écran

### 3.3 Workspace (Espace de travail)

| Propriété | Valeur |
|-----------|--------|
| `workspace-background` | `#1C1C1C` |
| `file-margins-x` | `32px` (size-4-8) |
| `file-margins-y` | `32px` (size-4-8) |
| `file-line-width` | `700px` |
| `file-folding-offset` | `24px` |
| `file-header-font` | `var(--font-interface)` |
| `file-header-font-size` | `var(--font-ui-small)` = `13px` |
| `file-header-font-weight` | `400` |
| `file-header-background` | `#1C1C1C` |
| `file-header-background-focused` | `#1C1C1C` |
| `file-header-border` | `1px solid transparent` |

### 3.4 Espacements

#### Système de Spacing (basé sur une grille de 2px et 4px)

| Variable | Valeur | Usage |
|----------|--------|-------|
| `size-2-1` | `2px` | Micro-espace |
| `size-2-2` | `4px` | Très petit espace |
| `size-2-3` | `6px` | Petit espace |
| `size-4-1` | `4px` | Espace standard 1 |
| `size-4-2` | `8px` | Espace standard 2 |
| `size-4-3` | `12px` | Espace standard 3 |
| `size-4-4` | `16px` | Espace standard 4 |
| `size-4-5` | `20px` | Espace standard 5 |
| `size-4-6` | `24px` | Espace standard 6 |
| `size-4-8` | `32px` | Grand espace |
| `size-4-9` | `36px` | Très grand espace |
| `size-4-10` | `40px` | Espace extra-large |
| `size-4-12` | `48px` | Espace très grand |
| `size-4-16` | `64px` | Espace maximum |
| `size-4-18` | `72px` | Espace absolu |

### 3.5 Bordures et Rayons

| Propriété | Valeur |
|-----------|--------|
| `border-width` | `1px` |
| `radius-s` | `4px` |
| `radius-m` | `8px` |
| `radius-l` | `12px` |
| `radius-xl` | `24px` (ou 18px selon contexte) |
| `corner-shape` | `[A VERIFIER]` |

#### Bordures spécifiques

| Élément | Couleur | Largeur | Rayon |
|---------|---------|---------|-------|
| Bordure principale | `#3f3f3f` | `1px` | - |
| Bordure hover | `#555555` | `1px` | - |
| Bordure focus | `#555555` | `1px` | - |
| Bordure séparateur | `#52525e` | `1px` | - |

---

## 4. Composants UI

### 4.1 Boutons

#### Bouton Principal (Primary)

| Propriété | Valeur |
|-----------|--------|
| `background` | `#027aff` |
| `background-hover` | `#80ebff` |
| `background-active` | `#aaf2ff` |
| `foreground` | `#fbfbfe` |
| `border` | `1px solid #027aff` |
| `border-radius` | `4px` |
| `height` | `32px` |
| `padding` | `4px 16px` |
| `font-size` | `13px` |
| `font-weight` | `590` |
| `disabled-opacity` | `0.4` |

#### Bouton Secondaire (Secondary)

| Propriété | Valeur |
|-----------|--------|
| `background` | `rgba(21, 20, 26, 0.07)` |
| `background-hover` | `rgba(21, 20, 26, 0.14)` |
| `background-active` | `rgba(21, 20, 26, 0.21)` |
| `foreground` | `#fbfbfe` |
| `border` | `1px solid rgba(21, 20, 26, 0.07)` |
| `border-radius` | `4px` |
| `height` | `32px` |
| `padding` | `4px 16px` |
| `font-size` | `13px` |
| `font-weight` | `590` |

#### Bouton avec Icône

| Propriété | Valeur |
|-----------|--------|
| `icon-size` | `16px` (m) / `14px` (xs) / `18px` (l) |
| `icon-stroke-width` | `2px` (m) / `1.75px` (l) / `1.25px` (xl) |
| `icon-color` | `#999999` |
| `icon-color-hover` | `#999999` |
| `icon-color-active` | `#44cf6e` |
| `icon-opacity` | `0.85` |
| `icon-opacity-hover` | `1` |
| `clickable-icon-radius` | `4px` |

### 4.2 Inputs et Champs de Texte

| Propriété | Valeur |
|-----------|--------|
| `input-height` | `30px` |
| `input-padding` | `4px 8px` |
| `input-radius` | `5px` |
| `input-border-width` | `1px` |
| `input-border-color` | `#8f8f9d` |
| `input-background` | `#212121` |
| `input-foreground` | `#fbfbfe` |
| `input-placeholder-color` | `#999999` |
| `input-font-size` | `13px` |
| `input-font-weight` | `400` |

### 4.3 É tomatoes (Code Blocks)

| Propriété | Valeur |
|-----------|--------|
| `code-background` | `#232323` |
| `code-border-width` | `0px` |
| `code-border-color` | `#3f3f3f` |
| `code-radius` | `4px` |
| `code-size` | `0.875em` |
| `code-bracket-background` | `#2e2e2e` |
| `code-white-space` | `pre-wrap` |
| `code-font-family` | JetBrains Mono (recommandé) |

### 4.4 Liens

| Propriété | Valeur |
|-----------|--------|
| `link-color` | `#027aff` |
| `link-color-hover` | `#80ebff` |
| `link-decoration` | `[A VERIFIER]` |
| `link-transition` | `[A VERIFIER]` |

### 4.5 Navigation

| Propriété | Valeur |
|-----------|--------|
| `nav-item-size` | `13px` (font-ui-small) |
| `nav-item-padding` | `4px 8px 4px 24px` |
| `nav-item-radius` | `8px` (radius-m) |
| `nav-item-color` | `#999999` (text-muted) |
| `nav-item-color-hover` | `#dadada` (text-normal) |
| `nav-item-color-active` | `#dadada` |
| `nav-item-color-selected` | `#dadada` |
| `nav-item-background-hover` | `#2e2e2e` (color-base-25) |
| `nav-item-background-active` | `#2e2e2e` |
| `nav-item-background-selected` | `color-mix(in oklch, #027aff 15%, transparent)` |
| `nav-item-margin-bottom` | `4px` (size-2-2) |
| `nav-item-weight` | `400` (font-normal) |

#### Navigation Headings

| Propriété | Valeur |
|-----------|--------|
| `nav-heading-color` | `#dadada` |
| `nav-heading-color-hover` | `#dadada` |
| `nav-heading-color-collapsed` | `#666666` |
| `nav-heading-color-collapsed-hover` | `#999999` |
| `nav-heading-weight` | `500` (font-medium) |

### 4.6 Tags

| Propriété | Valeur |
|-----------|--------|
| `nav-tag-background` | `transparent` |
| `nav-tag-color` | `#666666` |
| `nav-tag-color-hover` | `#999999` |
| `nav-tag-color-active` | `#999999` |
| `nav-tag-radius` | `4px` |
| `nav-tag-weight` | `600` |

### 4.7 Callouts

| Propriété | Valeur |
|-----------|--------|
| `callout-border-width` | `0px` |
| `callout-border-opacity` | `0.25` |
| `callout-padding` | `12px 12px 12px 24px` |
| `callout-radius` | `4px` |
| `callout-blend-mode` | `[A VERIFIER]` |
| `callout-title-color` | `inherit` |
| `callout-title-padding` | `0` |
| `callout-title-size` | `inherit` |
| `callout-title-weight` | `600` |
| `callout-content-padding` | `0` |
| `callout-content-background` | `transparent` |

### 4.8 Modal / Dialogues

| Propriété | Valeur |
|-----------|--------|
| `dialog-width` | `560px` |
| `dialog-max-width` | `80vw` |
| `dialog-max-height` | `85vh` |
| `dialog-background` | `#1c1b22` |
| `dialog-border-color` | `#1c1b22` |
| `dialog-border` | `1px solid #1c1b22` |
| `dialog-shadow` | `0 2px 14px 0 rgba(58, 57, 68, 0.2)` |
| `dialog-radius` | `4px` |
| `modal-background` | `#1c1b22` |
| `modal-header-height` | `40px` |

### 4.9 Tooltips

| Propriété | Valeur |
|-----------|--------|
| `tooltip-background` | `#FAFAFA` |
| `tooltip-color` | `[A VERIFIER]` |
| `tooltip-padding` | `[A VERIFIER]` |
| `tooltip-radius` | `[A VERIFIER]` |

### 4.10 Messages

| Propriété | Valeur |
|-----------|--------|
| `message-bar-background` | `#5a3100` |
| `message-bar-foreground` | `#fbfbfe` |
| `message-bar-border` | `1px solid rgba(255, 255, 255, 0.08)` |
| `message-bar-icon-color` | `#e49c49` |
| `message-bar-radius` | `4px` |
| `message-bar-padding` | `8px 8px 8px 16px` |

### 4.11 Toggles (Interrupteurs)

| Propriété | Valeur |
|-----------|--------|
| `toggle-width` | `40px` |
| `toggle-radius` | `18px` |
| `toggle-thumb-color` | `white` |
| `toggle-thumb-radius` | `18px` |
| `toggle-thumb-height` | `18px` |
| `toggle-thumb-width` | `18px` |
| `toggle-border-width` | `2px` |

#### Small Toggle

| Propriété | Valeur |
|-----------|--------|
| `toggle-s-width` | `34px` |
| `toggle-s-thumb-height` | `15px` |
| `toggle-s-thumb-width` | `15px` |
| `toggle-s-border-width` | `2px` |

### 4.12 File Tree (Arborescence des fichiers)

| Propriété | Valeur |
|-----------|--------|
| `nav-indentation-guide-width` | `1px` |
| `nav-indentation-guide-color` | `rgba(34, 34, 34, 0.12)` |
| `nav-indentation-guide-color-active` | `rgba(34, 34, 34, 0.30)` |
| `nav-collapse-icon-color` | `#666666` |
| `nav-collapse-icon-color-collapsed` | `#999999` |

---

## 5. Animations et Interactions

### 5.1 Durations

| Nom | Valeur | Usage |
|-----|--------|-------|
| `anim-duration-none` | `0ms` | Aucune animation |
| `anim-duration-superfast` | `70ms` | Très rapide |
| `anim-duration-fast` | `140ms` | Rapide |
| `anim-duration-moderate` | `300ms` | Moderne |
| `anim-duration-slow` | `560ms` | Lent |

### 5.2 Easing Functions

| Nom | Valeur | Description |
|-----|--------|-------------|
| `anim-motion-smooth` | `cubic-bezier(0.45, 0.05, 0.55, 0.95)` | Mouvement fluide |
| `anim-motion-delay` | `cubic-bezier(0.65, 0.05, 0.36, 1)` | Délai initial |
| `anim-motion-jumpy` | `cubic-bezier(0.68, -0.55, 0.27, 1.55)` | Rebond |
| `anim-motion-swing` | `cubic-bezier(0, 0.55, 0.45, 1)` | Balance |
| `anim-motion-spring` | `cubic-bezier(0.36, 0.7, 0, 1)` | Ressort |

### 5.3 Transitions

#### Hover

| Propriété | Valeur |
|-----------|--------|
| `filter` | `brightness(0.9)` |
| `duration` | `70ms` |
| `easing` | `cubic-bezier(0.45, 0.05, 0.55, 0.95)` |

#### Button Transitions

| Propriété | Durée |
|-----------|-------|
| Background | `140ms` |
| Border | `140ms` |
| Color | `140ms` |

#### Input Transitions

| Propriété | Durée |
|-----------|-------|
| Border | `140ms` |
| Box-shadow | `140ms` |

### 5.4 Drag and Drop

| Propriété | Valeur |
|-----------|--------|
| `drag-ghost-background` | `rgba(0, 0, 0, 0.85)` |
| `drag-ghost-text-color` | `#fff` |
| `drag-item-background` | `#1C1C1C` |
| `drag-item-shadow` | `0 2px 8px [A VERIFIER]` |
| `transition` | `140ms` |

### 5.5 Focus

| Propriété | Valeur |
|-----------|--------|
| `focus-ring-color` | `#0df` |
| `focus-ring-outline` | `2px solid var(--focus-ring-color)` |
| `focus-ring-outline-offset` | `2px` |
| `focus-transition` | `140ms` |

### 5.6 Click Interactions

| État | Transformation | Transition |
|------|----------------|------------|
| Active | `scale(0.95)` | `70ms` |
| Hover | `opacity: 0.85 to 1` | `70ms` |

### 5.7 Inline Title

| Propriété | Valeur |
|-----------|--------|
| `inline-title-color` | `var(--h1-color)` |
| `inline-title-font` | `var(--h1-font)` |
| `inline-title-line-height` | `var(--h1-line-height)` |
| `inline-title-size` | `var(--h1-size)` |
| `inline-title-style` | `var(--h1-style)` |
| `inline-title-weight` | `var(--h1-weight)` |
| `inline-title-margin-bottom` | `0.5em` |

---

## 6. Graphe de Connaissances

### 6.1 Couleurs

| Élément | Couleur | Code HEX |
|---------|---------|----------|
| `graph-background` | `var(--background-primary)` | `#1C1C1C` |
| `graph-text` | `var(--text-normal)` | `#dadada` |
| `graph-line` | `var(--color-base-35)` | `#3f3f3f` |
| `graph-node` | `var(--text-muted)` | `#999999` |
| `graph-node-unresolved` | `var(--text-faint)` | `#666666` |
| `graph-node-focused` | `var(--text-accent)` | `#44cf6e` |
| `graph-node-tag` | `var(--color-green)` | `#44cf6e` |
| `graph-node-attachment` | `var(--color-yellow)` | `#e0de71` |

### 6.2 Dimensions

| Propriété | Valeur |
|-----------|--------|
| `graph-controls-width` | `240px` |
| `canvas-controls-radius` | `4px` |
| `canvas-controls-icon-size` | `16px` |
| `canvas-controls-icon-stroke` | `2px` |
| `canvas-background` | `#1C1C1C` |
| `canvas-card-label-color` | `var(--text-faint)` | `#666666` |
| `canvas-dot-pattern` | `var(--color-base-30)` | `#333333` |

### 6.3 Comportement

- **Nœuds**: Représentent les notes
- **Liens**: Représentent les liens entre les notes
- **Focus**: Mise en évidence des nœuds sélectionnés
- **Tags**: Nœuds avec des tags spécifiques ont des couleurs dédiées
- **Pièces jointes**: Nœuds pour les fichiers attachés
- **Non résolus**: Nœuds pour les liens non résolus

---

## 7. Recommandations pour Note Craft

### 7.1 Implémentation des Couleurs

```css
/* Couleurs de base pour Note Craft */
:root {
  --nc-background-primary: #1C1C1C;
  --nc-background-secondary: #282828;
  --nc-text-primary: #dadada;
  --nc-text-secondary: #999999;
  --nc-text-muted: #666666;
  --nc-accent: #027aff;
  --nc-accent-hover: #80ebff;
  --nc-border: #3f3f3f;
}
```

### 7.2 Typographie

```css
/* Configuration pour JetBrains Mono */
:root {
  --nc-font-monospace: "JetBrains Mono", "Cascadia Mono", "Fira Code", monospace;
  --nc-font-interface: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  --nc-font-text: "Inter Variable", "Inter", "Segoe UI", sans-serif;
  
  /* Tailles */
  --nc-font-size-text: 16px;
  --nc-font-size-ui-small: 13px;
  --nc-font-size-ui-medium: 15px;
  --nc-font-size-code: 0.875em;
}
```

### 7.3 Layout des Panneaux Latéraux

```css
/* Configuration des sidebars */
:root {
  --nc-sidebar-width: 140px;
  --nc-sidebar-min-width: 140px;
  --nc-sidebar-max-width: 300px; /* À ajuster selon besoins */
  --nc-sidebar-background: #1C1C1C;
  --nc-sidebar-toggle-width-closed: 8.33%;
  --nc-sidebar-toggle-width-open: 24%;
  
  /* Espacements */
  --nc-spacing-xs: 4px;
  --nc-spacing-sm: 8px;
  --nc-spacing-md: 12px;
  --nc-spacing-lg: 16px;
  --nc-spacing-xl: 24px;
  --nc-spacing-2xl: 32px;
}

.note-craft-sidebar {
  width: var(--nc-sidebar-width);
  background: var(--nc-sidebar-background);
  transition: width 140ms cubic-bezier(0.45, 0.05, 0.55, 0.95);
}
```

### 7.4 Composants Clés

```css
/* Boutons */
.note-craft-button-primary {
  background: var(--nc-accent);
  color: var(--nc-text-primary);
  border: 1px solid var(--nc-accent);
  border-radius: 4px;
  height: 32px;
  padding: 4px 16px;
  font-size: var(--nc-font-size-ui-small);
  font-weight: 590;
  transition: background 140ms, border 140ms;
}

.note-craft-button-primary:hover {
  background: var(--nc-accent-hover);
}

/* Code blocks */
.note-craft-code-block {
  background: #232323;
  border-radius: 4px;
  padding: var(--nc-spacing-md);
  font-family: var(--nc-font-monospace);
  font-size: var(--nc-font-size-code);
  color: var(--nc-text-primary);
}

/* Navigation items */
.note-craft-nav-item {
  padding: 4px 8px 4px 24px;
  font-size: var(--nc-font-size-ui-small);
  color: var(--nc-text-muted);
  border-radius: 8px;
  transition: color 140ms, background 140ms;
}

.note-craft-nav-item:hover {
  color: var(--nc-text-primary);
  background: #2e2e2e;
}
```

### 7.5 Animations

```css
/* Animations de base */
:root {
  --nc-anim-fast: 140ms cubic-bezier(0.45, 0.05, 0.55, 0.95);
  --nc-anim-superfast: 70ms cubic-bezier(0.45, 0.05, 0.55, 0.95);
  --nc-anim-spring: cubic-bezier(0.36, 0.7, 0, 1);
}

/* Transition hover */
.note-craft-hoverable {
  transition: filter var(--nc-anim-superfast), 
              background var(--nc-anim-fast);
}

.note-craft-hoverable:hover {
  filter: brightness(0.9);
}

/* Focus */
.note-craft-focusable:focus {
  outline: 2px solid #0df;
  outline-offset: 2px;
}
```

### 7.6 Graphe de Connaissances

```css
.note-craft-graph {
  background: var(--nc-background-primary);
  color: var(--nc-text-primary);
}

.note-craft-graph-node {
  fill: var(--nc-text-muted);
  stroke: var(--nc-border);
}

.note-craft-graph-node-focused {
  fill: var(--nc-accent);
}

.note-craft-graph-line {
  stroke: var(--nc-border);
}
```

---

## Annexes

### A.1 Variables CSS Non Résolues (À Vérifier)

Les éléments suivants nécessitent une vérification supplémentaire dans le code source d'Obsidian :

- Largeur maximale des sidebars
- Largeur de la zone de travail centrale
- Comportement exact du drag & drop
- Animations spécifiques des onglets
- Style des tooltips
- Style des infra-bulles
- Comportement des popovers
- Style des badges
- Style des notifications

### A.2 Outils d'Extraction

Pour cette analyse, les outils suivants ont été utilisés :
- Extraction du binaire Obsidian 1.15.0
- Extraction des fichiers ASAR (app.asar, obsidian.asar)
- Analyse du fichier `app.css` principal
- Extraction des variables CSS via expressions régulières

### A.3 Limitations

1. Le fichier `app.css` est minifié et sur une seule ligne, ce qui rend l'extraction automatique plus complexe.
2. Certaines variables sont dynamiques et dépendent du contexte (thème clair/sombre).
3. Les valeurs spécifiques à certaines plateformes (macOS, Windows, Mobile) peuvent varier.
4. Les animations complexes et les comportements interactifs nécessitent une analyse dynamique.

### A.4 Références

- [Obsidian GitHub](https://github.com/obsidianmd/obsidian-releases)
- [Obsidian Developer Docs](https://github.com/obsidianmd/obsidian-developer-docs)
- [Obsidian API](https://github.com/obsidianmd/obsidian-api)

---

**Document généré le**: 2025-11-09  
**Version d'Obsidian analysée**: 1.15.0  
**Auteur**: Analyse automatique avec vérification humaine  
**Licence**: Ce document est destiné à un usage interne pour le projet Note Craft.
