;;; ind-util.el --- Transliteration and Misc. Tools for Indian Languages -*- coding: utf-8-emacs; lexical-binding: t; -*-

;; Copyright (C) 2001-2026 Free Software Foundation, Inc.

;; Keywords: multilingual, Indian, Devanagari

;; This file is part of femacs.

;; GNU Emacs is free software: you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; GNU Emacs is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.

;; You should have received a copy of the GNU General Public License
;; along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.

;;; Commentary:

;; This file provides conversion between UCS and various
;; transliteration schemes, such as ITRANS, kyoto-harvard and aiba
;; methods.  It also provides conversion between IS 13194 and UCS.
;; Finally, this program provides the compatibility support with
;; old implementation of Devanagari script.

;; Note: This file includes several codepoints outside of the Unicode
;; 0..#x10FFFF range, which are characters that were not unified into
;; Unicode.  Therefore, this file is encoded in utf-8-emacs, because
;; UTF-8 cannot encode such codepoints.  We include these codepoints
;; literally in the file to have them displayed by suitable fonts,
;; which makes maintenance easier.

;;; Code:

;;; Transliteration

;; The following provides the various transliteration schemes (such as
;; ITRANS, kyoto-harvard, and Aiba) of Indian scripts.  They are also
;; used in quail/indian.el for typing Indian script in Emacs.

(eval-and-compile

(defun indian-regexp-of-hashtbl-keys (hashtbl)
  "Return the regular expression of hash table keys."
  (let (keys)
    (maphash (lambda (key _val) (push key keys)) hashtbl)
    (regexp-opt keys)))

(defvar indian-dev-base-table
  '(
    (;; VOWELS  (18)
     (?à¤… nil) (?à¤† ?à¤¾) (?à¤‡ ?à¤¿) (?à¤ˆ ?à¥€) (?à¤‰ ?à¥) (?à¤Š ?à¥‚)
     (?à¤‹ ?à¥ƒ) (?à¤Œ ?à¥¢) (?à¤ ?à¥…) (?à¤Ž ?à¥†) (?à¤ ?à¥‡) (?à¤ ?à¥ˆ)
     (?à¤‘ ?à¥‰) (?à¤’ ?à¥Š) (?à¤“ ?à¥‹) (?à¤” ?à¥Œ) (?à¥  ?à¥„) (?à¥¡ ?à¥£))
    (;; CONSONANTS (currently 42, including special cases)
     ?à¤• ?à¤– ?à¤— ?à¤˜ ?à¤™                  ;; GUTTRULS
     ?à¤š ?à¤› ?à¤œ ?à¤ ?à¤ž                  ;; PALATALS
     ?à¤Ÿ ?à¤  ?à¤¡ ?à¤¢ ?à¤£                  ;; CEREBRALS
     ?à¤¤ ?à¤¥ ?à¤¦ ?à¤§ ?à¤¨ ?à¤©              ;; DENTALS
     ?à¤ª ?à¤« ?à¤¬ ?à¤­ ?à¤®                  ;; LABIALS
     ?à¤¯ ?à¤° ?à¤± ?à¤² ?à¤³ ?à¤´ ?à¤µ          ;; SEMIVOWELS
     ?à¤¶ ?à¤· ?à¤¸ ?à¤¹                    ;; SIBILANTS
     ?à¥˜ ?à¥™ ?à¥š ?à¥› ?à¥œ ?à¥ ?à¥ž ?à¥Ÿ      ;; NUKTAS
     "à¤œà¥à¤ž" "à¤•à¥à¤·")
    (;; Misc Symbols (7)
     ?à¤ ?à¤‚ ?à¤ƒ ?à¤½ ?à¥ ?à¥ ?à¥¤)
    (;; Digits (10)
     ?à¥¦ ?à¥§ ?à¥¨ ?à¥© ?à¥ª ?à¥« ?à¥¬ ?à¥­ ?à¥® ?à¥¯)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à¥à¤°" "à¤°à¥" "à¤¤à¥à¤°" "à¤¶à¥à¤°" "à¤¼")))

;; Punjabi is also known as Gurmukhi.
(defvar indian-pnj-base-table
  '(
    (;; VOWELS
     (?à¨… nil) (?à¨† ?à¨¾) (?à¨‡ ?à¨¿) (?à¨ˆ ?à©€) (?à¨‰ ?à©) (?à¨Š ?à©‚)
     nil nil nil nil (?à¨ ?à©‡) (?à¨ ?à©ˆ)
     nil nil (?à¨“ ?à©‹) (?à¨” ?à©Œ) nil nil)
    (;; CONSONANTS
     ?à¨• ?à¨– ?à¨— ?à¨˜ ?à¨™                  ;; GUTTRULS
     ?à¨š ?à¨› ?à¨œ ?à¨ ?à¨ž                  ;; PALATALS
     ?à¨Ÿ ?à¨  ?à¨¡ ?à¨¢ ?à¨£                  ;; CEREBRALS
     ?à¨¤ ?à¨¥ ?à¨¦ ?à¨§ ?à¨¨ nil              ;; DENTALS
     ?à¨ª ?à¨« ?à¨¬ ?à¨­ ?à¨®                  ;; LABIALS
     ?à¨¯ ?à¨° nil ?à¨² ?à¨³ nil ?à¨µ          ;; SEMIVOWELS
     ?à¨¶ nil ?à¨¸ ?à¨¹                    ;; SIBILANTS
     nil ?à©™ ?à©š ?à©› ?à©œ nil ?à©ž nil      ;; NUKTAS
     "à¨œà©à¨ž" nil)
    (;; Misc Symbols (7)
     nil ?à¨‚ nil nil ?à© nil nil) ;; ek onkar, etc.
    (;; Digits
     ?à©¦ ?à©§ ?à©¨ ?à©© ?à©ª ?à©« ?à©¬ ?à©­ ?à©® ?à©¯)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à©à¨°" "à¨°à©" "à¨¤à©à¨°" "à¨¶à©à¨°" "à¨¼")))

(defvar indian-gjr-base-table
  '(
    (;; VOWELS
     (?àª… nil) (?àª† ?àª¾) (?àª‡ ?àª¿) (?àªˆ ?à«€) (?àª‰ ?à«) (?àªŠ ?à«‚)
     (?àª‹ ?à«ƒ) nil (?àª ?à«…) nil (?àª ?à«‡) (?àª ?à«ˆ)
     (?àª‘ ?à«‰) nil (?àª“ ?à«‹) (?àª” ?à«Œ) (?à«  ?à«„) nil)
    (;; CONSONANTS
     ?àª• ?àª– ?àª— ?àª˜ ?àª™                  ;; GUTTRULS
     ?àªš ?àª› ?àªœ ?àª ?àªž                  ;; PALATALS
     ?àªŸ ?àª  ?àª¡ ?àª¢ ?àª£                  ;; CEREBRALS
     ?àª¤ ?àª¥ ?àª¦ ?àª§ ?àª¨ nil              ;; DENTALS
     ?àªª ?àª« ?àª¬ ?àª­ ?àª®                  ;; LABIALS
     ?àª¯ ?àª° nil ?àª² ?àª³ nil ?àªµ          ;; SEMIVOWELS
     ?àª¶ ?àª· ?àª¸ ?àª¹                    ;; SIBILANTS
     nil nil nil nil nil nil nil nil      ;; NUKTAS
     "àªœà«àªž" "àª•à«àª·")
    (;; Misc Symbols (7)
     ?àª ?àª‚ ?àªƒ ?àª½ ?à« ?à« nil)
    (;; Digits
     ?à«¦ ?à«§ ?à«¨ ?à«© ?à«ª ?à«« ?à«¬ ?à«­ ?à«® ?à«¯)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à«àª°" "àª°à«" "àª¤à«àª°" "àª¶à«àª°" "àª¼")))

(defvar indian-ori-base-table
  '(
    (;; VOWELS
     (?à¬… nil) (?à¬† ?à¬¾) (?à¬‡ ?à¬¿) (?à¬ˆ ?à­€) (?à¬‰ ?à­) (?à¬Š ?à­‚)
     (?à¬‹ ?à­ƒ) (?à¬Œ nil) nil nil (?à¬ ?à­‡) (?à¬ ?à­ˆ)
     nil nil (?à¬“ ?à­‹) (?à¬” ?à­Œ) (?à­  nil) (?à­¡ nil))
    (;; CONSONANTS
     ?à¬• ?à¬– ?à¬— ?à¬˜ ?à¬™                  ;; GUTTRULS
     ?à¬š ?à¬› ?à¬œ ?à¬ ?à¬ž                  ;; PALATALS
     ?à¬Ÿ ?à¬  ?à¬¡ ?à¬¢ ?à¬£                  ;; CEREBRALS
     ?à¬¤ ?à¬¥ ?à¬¦ ?à¬§ ?à¬¨ nil              ;; DENTALS
     ?à¬ª ?à¬« ?à¬¬ ?à¬­ ?à¬®                  ;; LABIALS
     ?à¬¯ ?à¬° nil ?à¬² ?à¬³ nil nil          ;; SEMIVOWELS
     ?à¬¶ ?à¬· ?à¬¸ ?à¬¹                    ;; SIBILANTS
     nil nil nil nil ?à­œ ?à­ nil ?à­Ÿ      ;; NUKTAS
     "à¬œà­à¬ž" "à¬•à­à¬·")
    (;; Misc Symbols
     ?à¬ ?à¬‚ ?à¬ƒ ?à¬½ ?à­ nil nil)
    (;; Digits
     ?à­¦ ?à­§ ?à­¨ ?à­© ?à­ª ?à­« ?à­¬ ?à­­ ?à­® ?à­¯)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à­à¬°" "à¬°à­" "à¬¤à­à¬°" "à¬¶à­à¬°" "à¬¼")))

(defvar indian-bng-base-table
  '(
    (;; VOWELS
     (?à¦… nil) (?à¦† ?à¦¾) (?à¦‡ ?à¦¿) (?à¦ˆ ?à§€) (?à¦‰ ?à§) (?à¦Š ?à§‚)
     (?à¦‹ ?à§ƒ) (?à¦Œ ?à§¢) nil nil (?à¦ ?à§‡) (?à¦ ?à§ˆ)
     nil nil (?à¦“ ?à§‹) (?à¦” ?à§Œ) (?à§  ?à§„) (?à§¡ ?à§£))
    (;; CONSONANTS
     ?à¦• ?à¦– ?à¦— ?à¦˜ ?à¦™                  ;; GUTTRULS
     ?à¦š ?à¦› ?à¦œ ?à¦ ?à¦ž                  ;; PALATALS
     ?à¦Ÿ ?à¦  ?à¦¡ ?à¦¢ ?à¦£                  ;; CEREBRALS
     ?à¦¤ ?à¦¥ ?à¦¦ ?à¦§ ?à¦¨ nil              ;; DENTALS
     ?à¦ª ?à¦« ?à¦¬ ?à¦­ ?à¦®                  ;; LABIALS
     ?à¦¯ ?à¦° nil ?à¦² nil nil nil          ;; SEMIVOWELS
     ?à¦¶ ?à¦· ?à¦¸ ?à¦¹                    ;; SIBILANTS
     nil nil nil nil ?à§œ ?à§ nil ?à§Ÿ      ;; NUKTAS
     "à¦œà§à¦ž" "à¦•à§à¦·")
    (;; Misc Symbols
     ?à¦ ?à¦‚ ?à¦ƒ nil ?à§ nil nil)
    (;; Digits
     ?à§¦ ?à§§ ?à§¨ ?à§© ?à§ª ?à§« ?à§¬ ?à§­ ?à§® ?à§¯)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à§à¦°" "à¦°à§" "à¦¤à§à¦°" "à¦¶à§à¦°" "à¦¼")))

(defvar indian-asm-base-table
  '(
    (;; VOWELS
     (?à¦… nil) (?à¦† ?à¦¾) (?à¦‡ ?à¦¿) (?à¦ˆ ?à§€) (?à¦‰ ?à§) (?à¦Š ?à§‚)
     (?à¦‹ ?à§ƒ) (?à¦Œ ?à§¢) nil nil (?à¦ ?à§‡) (?à¦ ?à§ˆ)
     nil nil (?à¦“ ?à§‹) (?à¦” ?à§Œ) (?à§  ?à§„) (?à§¡ ?à§£))
    (;; CONSONANTS
     ?à¦• ?à¦– ?à¦— ?à¦˜ ?à¦™                  ;; GUTTRULS
     ?à¦š ?à¦› ?à¦œ ?à¦ ?à¦ž                  ;; PALATALS
     ?à¦Ÿ ?à¦  ?à¦¡ ?à¦¢ ?à¦£                  ;; CEREBRALS
     ?à¦¤ ?à¦¥ ?à¦¦ ?à¦§ ?à¦¨ nil              ;; DENTALS
     ?à¦ª ?à¦« ?à¦¬ ?à¦­ ?à¦®                  ;; LABIALS
     ?à¦¯ ?à§° nil ?à¦² nil nil ?à§±          ;; SEMIVOWELS
     ?à¦¶ ?à¦· ?à¦¸ ?à¦¹                    ;; SIBILANTS
     nil nil nil nil ?à§œ ?à§ nil ?à§Ÿ      ;; NUKTAS
     "à¦œà§à¦ž" "à¦•à§à¦·")
    (;; Misc Symbols
     ?à¦ ?à¦‚ ?à¦ƒ nil ?à§ nil nil)
    (;; Digits
     ?à§¦ ?à§§ ?à§¨ ?à§© ?à§ª ?à§« ?à§¬ ?à§­ ?à§® ?à§¯)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à§à§°" "à§°à§" "à¦¤à§à§°" "à¦¶à§à§°" "à¦¼")))

(defvar indian-tlg-base-table
  '(
    (;; VOWELS
     (?à°… nil) (?à°† ?à°¾) (?à°‡ ?à°¿) (?à°ˆ ?à±€) (?à°‰ ?à±) (?à°Š ?à±‚)
     (?à°‹ ?à±ƒ) (?à°Œ nil) nil (?à° ?à±‡) (?à°Ž ?à±†) (?à° ?à±ˆ)
     nil (?à°“ ?à±‹) (?à°’ ?à±Š) (?à°” ?à±Œ) (?à±  ?à±„) (?à±¡ nil))
    (;; CONSONANTS
     ?à°• ?à°– ?à°— ?à°˜ ?à°™                  ;; GUTTRULS
     ?à°š ?à°› ?à°œ ?à° ?à°ž                  ;; PALATALS
     ?à°Ÿ ?à°  ?à°¡ ?à°¢ ?à°£                  ;; CEREBRALS
     ?à°¤ ?à°¥ ?à°¦ ?à°§ ?à°¨ nil              ;; DENTALS
     ?à°ª ?à°« ?à°¬ ?à°­ ?à°®                  ;; LABIALS
     ?à°¯ ?à°° ?à°± ?à°² ?à°³ nil ?à°µ          ;; SEMIVOWELS
     ?à°¶ ?à°· ?à°¸ ?à°¹                    ;; SIBILANTS
     nil nil nil nil nil nil nil nil      ;; NUKTAS
     "à°œà±à°ž" "à°•à±à°·")
    (;; Misc Symbols
     ?à° ?à°‚ ?à°ƒ nil ?à± nil nil)
    (;; Digits
     ?à±¦ ?à±§ ?à±¨ ?à±© ?à±ª ?à±« ?à±¬ ?à±­ ?à±® ?à±¯)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à±à°°" "à°°à±" "à°¤à±à°°" "à°¶à±à°°" nil)))

(defvar indian-knd-base-table
  '(
    (;; VOWELS
     (?à²… nil) (?à²† ?à²¾) (?à²‡ ?à²¿) (?à²ˆ ?à³€) (?à²‰ ?à³) (?à²Š ?à³‚)
     (?à²‹ ?à³ƒ) (?à²Œ nil) nil (?à² ?à³‡) (?à²Ž ?à³†) (?à² ?à³ˆ)
     nil (?à²“ ?à³‹) (?à²’ ?à³Š) (?à²” ?à³Œ) (?à³  ?à³„) (?à³¡ nil))
    (;; CONSONANTS
     ?à²• ?à²– ?à²— ?à²˜ ?à²™                  ;; GUTTRULS
     ?à²š ?à²› ?à²œ ?à² ?à²ž                  ;; PALATALS
     ?à²Ÿ ?à²  ?à²¡ ?à²¢ ?à²£                  ;; CEREBRALS
     ?à²¤ ?à²¥ ?à²¦ ?à²§ ?à²¨ nil              ;; DENTALS
     ?à²ª ?à²« ?à²¬ ?à²­ ?à²®                  ;; LABIALS
     ?à²¯ ?à²° ?à²± ?à²² ?à²³ nil ?à²µ          ;; SEMIVOWELS
     ?à²¶ ?à²· ?à²¸ ?à²¹                    ;; SIBILANTS
     nil nil nil nil nil nil ?à³ž nil      ;; NUKTAS
     "à²œà³à²ž" "à²•à³à²·")
    (;; Misc Symbols
     nil ?à²‚ ?à²ƒ nil ?à³ nil nil)
    (;; Digits
     ?à³¦ ?à³§ ?à³¨ ?à³© ?à³ª ?à³« ?à³¬ ?à³­ ?à³® ?à³¯)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à³à²°" "à²°à³" "à²¤à³à²°" "à²¶à³à²°" nil)))

(defvar indian-mlm-base-table
  '(
    (;; VOWELS
     (?à´… nil) (?à´† ?à´¾) (?à´‡ ?à´¿) (?à´ˆ ?àµ€) (?à´‰ ?àµ) (?à´Š ?àµ‚)
     (?à´‹ ?àµƒ) (?à´Œ ?àµ¢) (?àµ¡ ?àµ£) (?à´ ?àµ‡) (?à´Ž ?àµ†) (?à´ ?àµˆ)
     nil (?à´’ ?àµŠ) (?à´“ ?àµ‹) (?à´” ?àµ—) (?àµ ?àµ) (?àµ  ?àµ„))
    (;; CONSONANTS
     ?à´• ?à´– ?à´— ?à´˜ ?à´™                  ;; GUTTRULS
     ?à´š ?à´› ?à´œ ?à´ ?à´ž                  ;; PALATALS
     ?à´Ÿ ?à´  ?à´¡ ?à´¢ ?à´£                  ;; CEREBRALS
     ?à´¤ ?à´¥ ?à´¦ ?à´§ ?à´¨ nil              ;; DENTALS
     ?à´ª ?à´« ?à´¬ ?à´­ ?à´®                  ;; LABIALS
     ?à´¯ ?à´° ?à´± ?à´² ?à´³ ?à´´ ?à´µ          ;; SEMIVOWELS
     ?à´¶ ?à´· ?à´¸ ?à´¹                    ;; SIBILANTS
     nil nil nil nil nil nil nil nil      ;; NUKTAS
     "à´œàµà´ž" "à´•àµà´·"
     "à´±àµà´±" "à´¨àµà´±" "à´¤àµà´¤" "à´¤àµà´¥" "à´žàµà´ž" "à´™àµà´™" "à´¨àµà´¨"
     "à´žàµà´š" "à´¨àµà´•" "à´™àµà´•" "à´šàµà´š" "à´šàµà´›" "à´•àµà´•"
     "à´¬àµà´¬" "à´•àµà´•" "à´—àµà´—" "à´œàµà´œ" "à´®àµà´®" "à´ªàµà´ª" "à´µàµà´µ" "à´•àµà´¸" "à´¶àµà´¶")
    (;; Misc Symbols
     nil ?à´‚ ?à´ƒ nil ?àµ nil nil)
    (;; Digits
     ?àµ¦ ?àµ§ ?àµ¨ ?àµ© ?àµª ?àµ« ?àµ¬ ?àµ­ ?àµ® ?àµ¯)
    (;; Chillus
     "à´£àµ" ?àµº "à´¨àµ" ?àµ» "à´°àµ" ?àµ¼ "à´²àµ" ?àµ½ "à´³àµ" ?àµ¾)))

(defvar indian-tml-base-table
  '(
    (;; VOWELS
     (?à®… nil) (?à®† ?à®¾) (?à®‡ ?à®¿) (?à®ˆ ?à¯€) (?à®‰ ?à¯) (?à®Š ?à¯‚)
     nil nil nil (?à® ?à¯‡) (?à®Ž ?à¯†) (?à® ?à¯ˆ)
     nil (?à®“ ?à¯‹) (?à®’ ?à¯Š) (?à®” ?à¯Œ) nil nil)
    (;; CONSONANTS
     ?à®• nil nil nil ?à®™                  ;; GUTTRULS
     ?à®š nil ?à®œ nil ?à®ž                  ;; PALATALS
     ?à®Ÿ nil nil nil ?à®£                  ;; CEREBRALS
     ?à®¤ nil nil nil ?à®¨ ?à®©              ;; DENTALS
     ?à®ª nil nil nil ?à®®                  ;; LABIALS
     ?à®¯ ?à®° ?à®± ?à®² ?à®³ ?à®´ ?à®µ          ;; SEMIVOWELS
     ?à®¶ ?à®· ?à®¸ ?à®¹                    ;; SIBILANTS
     nil nil nil nil nil nil nil nil      ;; NUKTAS
     "à®œà¯à®ž" "à®•à¯à®·" "à®•à¯â€Œà®·")
    (;; Misc Symbols
     nil ?à®‚ ?à®ƒ nil ?à¯ ?à¯ nil)
    (;; Digits
     nil nil nil nil nil nil nil nil nil nil)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à¯à®°" "à®°à¯" "à®¤à¯à®°" nil nil)))

(defvar indian-tml-base-digits-table
  '(
    (;; VOWELS
     (?à®… nil) (?à®† ?à®¾) (?à®‡ ?à®¿) (?à®ˆ ?à¯€) (?à®‰ ?à¯) (?à®Š ?à¯‚)
     nil nil nil (?à® ?à¯‡) (?à®Ž ?à¯†) (?à® ?à¯ˆ)
     nil (?à®“ ?à¯‹) (?à®’ ?à¯Š) (?à®” ?à¯Œ) nil nil)
    (;; CONSONANTS
     ?à®• nil nil nil ?à®™                  ;; GUTTRULS
     ?à®š nil ?à®œ nil ?à®ž                  ;; PALATALS
     ?à®Ÿ nil nil nil ?à®£                  ;; CEREBRALS
     ?à®¤ nil nil nil ?à®¨ ?à®©              ;; DENTALS
     ?à®ª nil nil nil ?à®®                  ;; LABIALS
     ?à®¯ ?à®° ?à®± ?à®² ?à®³ ?à®´ ?à®µ          ;; SEMIVOWELS
     ?à®¶ ?à®· ?à®¸ ?à®¹                    ;; SIBILANTS
     nil nil nil nil nil nil nil nil      ;; NUKTAS
     "à®œà¯à®ž" "à®•à¯à®·" "à®•à¯â€Œà®·")
    (;; Misc Symbols
     nil ?à®‚ ?à®ƒ nil ?à¯ ?à¯ nil)
    (;; Digits
     ?à¯¦ ?à¯§ ?à¯¨ ?à¯© ?à¯ª ?à¯« ?à¯¬ ?à¯­ ?à¯® ?à¯¯)
    (;; Inscript-extra (4)  (#, $, ^, *, ])
     "à¯à®°" "à®°à¯" "à®¤à¯à®°" nil nil)))

(defvar indian-base-table-to-language-alist
  '((indian-dev-base-table . "Devanagari")
    (indian-pnj-base-table . "Punjabi")
    (indian-ori-base-table . "Oriya")
    (indian-bng-base-table . "Bengali")
    (indian-asm-base-table . "Assamese")
    (indian-tlg-base-table . "Telugu")
    (indian-knd-base-table . "Kannada")
    (indian-mlm-base-table . "Malayalam")
    (indian-tml-base-table . "Tamil")))

(defvar indian-itrans-v5-table
  '(;; for encode/decode
    (;; vowels -- 18
     "a" ("aa" "A") "i" ("ii" "I") "u" ("uu" "U")
     ("RRi" "R^i" "RRu" "R^u") ("LLi" "L^i") (".c" "e.c") "E" "e" "ai"
     "o.c"  "O"   "o"   "au"  ("RRI" "R^I" "RRU" "R^U") ("LLI" "L^I"))
    (;; consonants -- 40
     "k"   "kh"  "g"   "gh"  ("~N" "N^")
     "ch" ("Ch" "chh") "j" "jh" ("~n" "JN")
     "T"   "Th"  "D"   "Dh"  "N"
     "t"   "th"  "d"   "dh"  "n"   "nh"
     "p"   "ph"  "b"   "bh"  "m"
     "y"   "r"   "rh"  "l"   ("L" "ld") nil  ("v" "w")
     "sh" ("Sh" "shh") "s" "h"
     "q" "K" "G" ("J" "z") ".D" ".Dh" "f" ("Y" "yh")
     ("GY" "dny") "x")
    (;; misc -- 7
     ".N" (".n" "M") "H" ".a" ".h" ("AUM" "OM") "..")))

(defvar indian-itrans-v5-table-for-tamil
  '(;; for encode/decode
    (;; vowels -- 18
     "a" ("aa" "A") "i" ("ii" "I") "u" ("uu" "U")
     ("RRi" "R^i") ("LLi" "L^i") (".c" "e.c") "E" "e" "ai"
     "o.c"  "O"   "o"   "au"  ("RRI" "R^I") ("LLI" "L^I"))
    (;; consonants -- 40
     "k"   "kh"  "g"   "gh"  ("~N" "N^")
     "ch" ("Ch" "chh") "j" "jh" ("~n" "JN")
     "T"   "Th"  "D"   "Dh"  "N"
     "t"   "th"  "d"   "dh"  "n"   "nh"
     "p"   "ph"  "b"   "bh"  "m"
     "y"   "r"   "rh"  "l"   ("L" "ld") ("J" "z")  ("v" "w")
     "sh" ("Sh" "shh") "s" "h"
     "q" "K" "G" nil ".D" ".Dh" "f" ("Y" "yh")
     ("GY" "dny") "x")
    (;; misc -- 7
     ".N" (".n" "M") "H" ".a" ".h" ("AUM" "OM") "..")))

(defvar indian-mlm-mozhi-table
  '(;; for encode/decode
    (;; vowels -- 18
     "a" ("aa" "A") "i" ("ii" "I") "u" ("uu" "U")
     "R" "Ll" "Lll" ("E" "ae") "e" "ai"
     nil  "o"   "O"   "au"  "~" "RR")
    (;; consonants -- 40
     ("k" "c")   "kh"  "g"   "gh"  "ng"
     "ch" ("Ch" "chh") "j" "jh" "nj"
     "T"   "Th"  "D"   "Dh"  "N"
     "th"  "thh" "d"   "dh"  "n"   nil
     "p"   ("ph" "f")  "b"   "bh"  "m"
     "y"   "r"   "rr"  "l"  "L" "zh" ("v" "w")
     ("S" "z") "sh" "s" "h"
     nil nil nil nil nil nil nil nil
     nil "X"
     ;; some of these are extra to Mozhi
     ("t" "tt") "nt" "tth" "tthh" "nnj" "nng" "nn"
     "nch" "nc" "nk" "cch" "cchh" "cc"
     "B" ("C" "K" "q") "G" "J" "M" "P" "V" "x" "Z")
    (;; misc -- 7
     nil nil "H")))

(defvar indian-kyoto-harvard-table
  '(;; for encode/decode
    (;; vowel
     "a"   ("A" "aa")  "i"   ("I" "ii")  "u"   ("U" "uu")
     "R"   ("L" "lR")  nil   nil   "e"   "ai"
     nil   nil   "o"   "au"  ("q" "RR" "Q")   ("E" "LL" "lRR"))
    (;; consonant
     "k"   "kh"  "g"   "gh"  "G"
     "c"   "ch"  "j"   "jh"  "J"
     "T"   "Th"  "D"   "Dh"  "N"
     "t"   "th"  "d"   "dh"  "n"   nil
     "p"   "ph"  "b"   "bh"  "m"
     "y"   "r"   nil   "l"   "L"   nil   "v"
     ("z" "Z")   "S"   "s"   "h"
     nil   nil   nil   nil   nil   nil   nil   nil
     nil   nil)
    (;; misc
     nil   "M"   "H"   "'"   nil   "." nil)))

(defvar indian-harvard-table
  '(;; for encode/decode
    (;; vowel
     "a"   ("A" "aa")  "i"   ("I" "ii")  "u"   ("U" "uu")
     "R"   ("L" "lR")  nil   nil   "e"   "ai"
     nil   nil   "o"   "au"  ("RR" "q" "Q")   ("LL" "E" "lRR"))
    (;; consonant
     "k"   "kh"  "g"   "gh"  "G"
     "c"   "ch"  "j"   "jh"  "J"
     "T"   "Th"  "D"   "Dh"  "N"
     "t"   "th"  "d"   "dh"  "n"   nil
     "p"   "ph"  "b"   "bh"  "m"
     "y"   "r"   nil   "l"   "L"   nil   "v"
     ("z" "Z")   "S"   "s"   "h"
     nil   nil   nil   nil   nil   nil   nil   nil
     nil   nil)
    (;; misc
     nil   "M"   "H"   "'"   nil   "." nil)))

(defvar indian-tokyo-table
  '(;; for encode/decode
    (;; vowel
     "a"   ("A" "aa")  "i"   ("I" "ii")  "u"   ("U" "uu")
     "R"   ("L" "lR")  nil   nil   "e"   "ai"
     nil   nil   "o"   "au"  ("Q" "RR" "q")   ("E" "LL" "lRR"))
    (;; consonant
     "k"   "kh"  "g"   "gh"  "G"
     "c"   "ch"  "j"   "jh"  "J"
     "T"   "Th"  "D"   "Dh"  "N"
     "t"   "th"  "d"   "dh"  "n"   nil
     "p"   "ph"  "b"   "bh"  "m"
     "y"   "r"   nil   "l"   "L"   nil   "v"
     ("Z" "z")   "S"   "s"   "h"
     nil   nil   nil   nil   nil   nil   nil   nil
     nil   nil)
    (;; misc
     nil   "M"   "H"   "'"   nil   "." nil)))

(defvar indian-aiba-table
  '(;; for encode/decode
    (;; vowel
     "a"   "aa"  "i"   "ii"  "u"   "uu"
     ".r"  ".l"   nil   nil  "e"   "ai"
     nil   nil   "o"   "au"  "~r"  "~l")
    (;; consonant
     "k"   "kh"  "g"   "gh"  "^n"
     "c"   "ch"  "j"   "jh"  "~n"
     ".t"  ".th" ".d"  ".dh" ".n"
     "t"   "th"  "d"   "dh"  "n"   nil
     "p"   "ph"  "b"   "bh"  "m"
     "y"   "r"   nil   "l"   nil  nil  "v"
     "^s"  ".s"  "s"   "h"
     nil   nil   nil   nil   nil   nil   nil   nil
     nil   nil)
    (;; misc
     nil   ".m"  ".h"  "'"   nil   "." nil)))

(defun combinatorial (head &rest tail)
  (if tail
      (apply 'append
	     (mapcar (lambda (y) (mapcar (lambda (x) (cons x y)) head))
		     (apply 'combinatorial tail)))
    (mapcar 'list head)))

(defun indian--puthash-char (char trans-char hashtbls)
  (let ((encode-hash (car hashtbls))  ;; char -> trans
	(decode-hash (cdr hashtbls))  ;; trans -> char
	)
    ;; char -- nil / char / string (/ list of vowel & matra)
    ;; trans-char -- nil / string / list of strings
    (when (and char trans-char)
      (if (stringp trans-char) (setq trans-char (list trans-char)))
      (if (characterp char) (setq char (char-to-string char)))
      (puthash char (car trans-char) encode-hash)
      (dolist (trans trans-char)
	 (puthash trans char decode-hash)))))

(defun indian--map (f l1 l2)
  (while l1
    (funcall f (pop l1) (pop l2))))

(defun indian--puthash-v (v trans-v hashtbls)
  (indian--map
   (lambda (v trans-v)
     (indian--puthash-char (car v) trans-v hashtbls))
   v trans-v))

(defun indian--puthash-c (c trans-c halant hashtbls)
  (indian--map
   (lambda (c trans-c)
     (if (characterp c) (setq c (char-to-string c)))
     (indian--puthash-char (concat c halant) trans-c hashtbls))
   c trans-c))

(defun indian--puthash-m (m trans-m hashtbls)
  (indian--map
   (lambda (m trans-m)
     (indian--puthash-char m trans-m hashtbls))
   m trans-m))

(defun indian--puthash-cv (c trans-c v trans-v hashtbls)
  (indian--map
   (lambda (c trans-c)
     (indian--map
      (lambda (v trans-v)
	(when (and c trans-c  v trans-v)
	  (if (characterp c) (setq c (char-to-string c)))
	  (setq v (if (characterp (cadr v)) (char-to-string (cadr v)) ""))
	  (if (stringp trans-c) (setq trans-c (list trans-c)))
	  (if (stringp trans-v) (setq trans-v (list trans-v)))
	  (indian--puthash-char
	   (concat c v)
	   (mapcar (lambda (x) (apply 'concat x))
		  (combinatorial trans-c trans-v))
	   hashtbls)))
      v trans-v))
   c trans-c))

(defun indian-make-hash (table trans-table)
  "Indian Transliteration Hash for decode/encode."
  (let* ((encode-hash (make-hash-table :test 'equal))
	 (decode-hash (make-hash-table :test 'equal))
	 (hashtbls (cons encode-hash decode-hash))
	 (vowels     (elt table 0))
	 (consonants (elt table 1))
	 (misc       (elt table 2))
	 (digits     (elt table 3))
	 (halant     (char-to-string (elt misc  4)))
	 (trans-vowels     (elt trans-table 0))
	 (trans-consonants (elt trans-table 1))
	 (trans-misc       (elt trans-table 2))
	 (trans-digits  '("0" "1" "2" "3" "4" "5" "6" "7" "8" "9")))
    (indian--puthash-v vowels trans-vowels hashtbls)
    (indian--puthash-c consonants trans-consonants halant hashtbls)
    (indian--puthash-cv consonants trans-consonants
			      vowels trans-vowels hashtbls)
    (indian--puthash-m misc trans-misc hashtbls)
    (indian--puthash-m digits trans-digits hashtbls)
    hashtbls))

(defvar indian-dev-itrans-v5-hash
  (indian-make-hash indian-dev-base-table
			  indian-itrans-v5-table))
(defvar indian-dev-kyoto-harvard-hash
  (indian-make-hash indian-dev-base-table
			  indian-kyoto-harvard-table))
(defvar indian-dev-aiba-hash
  (indian-make-hash indian-dev-base-table
			  indian-aiba-table))

(defvar indian-pnj-itrans-v5-hash
  (indian-make-hash indian-pnj-base-table
			  indian-itrans-v5-table))

(defvar indian-gjr-itrans-v5-hash
  (indian-make-hash indian-gjr-base-table
			  indian-itrans-v5-table))

(defvar indian-ori-itrans-v5-hash
  (indian-make-hash indian-ori-base-table
			  indian-itrans-v5-table))

(defvar indian-bng-itrans-v5-hash
  (indian-make-hash indian-bng-base-table
			  indian-itrans-v5-table))

(defvar indian-asm-itrans-v5-hash
  (indian-make-hash indian-asm-base-table
			  indian-itrans-v5-table))

(defvar indian-tlg-itrans-v5-hash
  (indian-make-hash indian-tlg-base-table
			  indian-itrans-v5-table))

(defvar indian-knd-itrans-v5-hash
  (indian-make-hash indian-knd-base-table
			  indian-itrans-v5-table))

(defvar indian-mlm-itrans-v5-hash
  (indian-make-hash indian-mlm-base-table
			  indian-itrans-v5-table))

(defvar indian-mlm-mozhi-hash
  (indian-make-hash indian-mlm-base-table
			  indian-mlm-mozhi-table))

(defvar indian-tml-itrans-v5-hash
  (indian-make-hash indian-tml-base-table
			  indian-itrans-v5-table-for-tamil))

(defvar indian-tml-itrans-digits-v5-hash
  (indian-make-hash indian-tml-base-digits-table
			  indian-itrans-v5-table-for-tamil))
)

(defmacro indian-translate-region (from to hashtable encode-p)
  `(save-excursion
     (save-restriction
       (let ((regexp ,(indian-regexp-of-hashtbl-keys
		       (if encode-p (car (eval hashtable))
			 (cdr (eval hashtable))))))
	 (narrow-to-region ,from ,to)
	 (goto-char (point-min))
	 (while (re-search-forward regexp nil t)
	   (let ((matchstr (gethash (match-string 0)
				    (if ,encode-p
					(car ,hashtable)
				      (cdr ,hashtable)))))
	     (if matchstr (replace-match matchstr))))))))

;;;

(defun indian-dev-itrans-v5-encode-region (from to)
  (interactive "r")
  (indian-translate-region
   from to indian-dev-itrans-v5-hash t))

(defun indian-dev-itrans-v5-decode-region (from to)
  (interactive "r")
  (indian-translate-region
   from to indian-dev-itrans-v5-hash nil))

(defun indian-dev-kyoto-harvard-encode-region (from to)
  (interactive "r")
  (indian-translate-region
   from to indian-dev-kyoto-harvard-hash t))

(defun indian-dev-kyoto-harvard-decode-region (from to)
  (interactive "r")
  (indian-translate-region
   from to indian-dev-kyoto-harvard-hash nil))

(defun indian-dev-aiba-encode-region (from to)
  (interactive "r")
  (indian-translate-region
   from to indian-dev-aiba-hash t))

(defun indian-dev-aiba-decode-region (from to)
  (interactive "r")
  (indian-translate-region
   from to indian-dev-aiba-hash nil))




;;; IS 13194 utilities

;; The following provides conversion between IS 13194 (ISCII) and UCS.

(dlet
    ;;Unicode vs IS13194  ;; only Devanagari is supported now.
    ((ucs-devanagari-to-is13194-alist
      '((?\x0900 . "[U+0900]")
	(?\x0901 . "ö€€€")
	(?\x0902 . "ö€€")
	(?\x0903 . "ö€€‚")
	(?\x0904 . "[U+0904]")
	(?\x0905 . "ö€€ƒ")
	(?\x0906 . "ö€€„")
	(?\x0907 . "ö€€…")
	(?\x0908 . "ö€€†")
	(?\x0909 . "ö€€‡")
	(?\x090a . "ö€€ˆ")
	(?\x090b . "ö€€‰")
	(?\x090c . "ö€€…ö€ˆ")
	(?\x090d . "ö€€")
	(?\x090e . "ö€€Š")
	(?\x090f . "ö€€‹")
	(?\x0910 . "ö€€Œ")
	(?\x0911 . "ö€€‘")
	(?\x0912 . "ö€€Ž")
	(?\x0913 . "ö€€")
	(?\x0914 . "ö€€")
	(?\x0915 . "ö€€’")
	(?\x0916 . "ö€€“")
	(?\x0917 . "ö€€”")
	(?\x0918 . "ö€€•")
	(?\x0919 . "ö€€–")
	(?\x091a . "ö€€—")
	(?\x091b . "ö€€˜")
	(?\x091c . "ö€€™")
	(?\x091d . "ö€€š")
	(?\x091e . "ö€€›")
	(?\x091f . "ö€€œ")
	(?\x0920 . "ö€€")
	(?\x0921 . "ö€€ž")
	(?\x0922 . "ö€€Ÿ")
	(?\x0923 . "ö€€ ")
	(?\x0924 . "ö€€¡")
	(?\x0925 . "ö€€¢")
	(?\x0926 . "ö€€£")
	(?\x0927 . "ö€€¤")
	(?\x0928 . "ö€€¥")
	(?\x0929 . "ö€€¦")
	(?\x092a . "ö€€§")
	(?\x092b . "ö€€¨")
	(?\x092c . "ö€€©")
	(?\x092d . "ö€€ª")
	(?\x092e . "ö€€«")
	(?\x092f . "ö€€¬")
	(?\x0930 . "ö€€®")
	(?\x0931 . "ö€€¯")
	(?\x0932 . "ö€€°")
	(?\x0933 . "ö€€±")
	(?\x0934 . "ö€€²")
	(?\x0935 . "ö€€³")
	(?\x0936 . "ö€€´")
	(?\x0937 . "ö€€µ")
	(?\x0938 . "ö€€¶")
	(?\x0939 . "ö€€·")
	(?\x093a . "[U+093a]")
	(?\x093b . "[U+093b]")
	(?\x093c . "ö€ˆ")
	(?\x093d . "ö€‰ö€ˆ")
	(?\x093e . "ö€€¹")
	(?\x093f . "ö€€º")
	(?\x0940 . "ö€€»")
	(?\x0941 . "ö€€¼")
	(?\x0942 . "ö€€½")
	(?\x0943 . "ö€€¾")
	(?\x0944 . "ö€€¾ö€ˆ")
	(?\x0945 . "ö€‚")
	(?\x0946 . "ö€€¿")
	(?\x0947 . "ö€€")
	(?\x0948 . "ö€")
	(?\x0949 . "ö€†")
	(?\x094a . "ö€ƒ")
	(?\x094b . "ö€„")
	(?\x094c . "ö€…")
	(?\x094d . "ö€‡")
	(?\x094e . "[U+094e]")
	(?\x094f . "[U+094f]")
	(?\x0950 . "ö€€€ö€ˆ")
	(?\x0951 . "ö€ö€€”")
	(?\x0952 . "ö€ö€€—")
	(?\x0953 . "[DEVANAGARI GRAVE ACCENT]")
	(?\x0954 . "[DEVANAGARI ACUTE ACCENT]")
	(?\x0955 . "[U+0955]")
	(?\x0956 . "[U+0956]")
	(?\x0957 . "[U+0957]")
	(?\x0958 . "ö€€’ö€ˆ")
	(?\x0959 . "ö€€“ö€ˆ")
	(?\x095a . "ö€€”ö€ˆ")
	(?\x095b . "ö€€™ö€ˆ")
	(?\x095c . "ö€€žö€ˆ")
	(?\x095d . "ö€€Ÿö€ˆ")
	(?\x095e . "ö€€¨ö€ˆ")
	(?\x095f . "ö€€­")
	(?\x0960 . "ö€€‰ö€ˆ")
	(?\x0961 . "ö€€†ö€ˆ")
	(?\x0962 . "ö€€ºö€ˆ")
	(?\x0963 . "ö€„ö€ˆ")
	(?\x0964 . "ö€‰")
	(?\x0965 . "ö€‰ö€‰")
	(?\x0966 . "ö€")
	(?\x0967 . "ö€‘")
	(?\x0968 . "ö€’")
	(?\x0969 . "ö€“")
	(?\x096a . "ö€”")
	(?\x096b . "ö€•")
	(?\x096c . "ö€–")
	(?\x096d . "ö€—")
	(?\x096e . "ö€˜")
	(?\x096f . "ö€™")
	(?\x0970 . "[U+0970]")
	(?\x0971 . "[U+0971]")
	(?\x0972 . "[U+0972]")
	(?\x0973 . "[U+0973]")
	(?\x0974 . "[U+0974]")
	(?\x0975 . "[U+0975]")
	(?\x0976 . "[U+0976]")
	(?\x0977 . "[U+0977]")
	(?\x0978 . "[U+0978]")
	(?\x0979 . "[U+0979]")
	(?\x097a . "[U+097a]")
	(?\x097b . "[U+097b]")
	(?\x097c . "[U+097c]")
	(?\x097d . "[U+097d]")
	(?\x097e . "[U+097e]")
	(?\x097f . "[U+097f]")))
     (ucs-bengali-to-is13194-alist nil)
     (ucs-assamese-to-is13194-alist nil)
     (ucs-gurmukhi-to-is13194-alist nil)
     (ucs-gujarati-to-is13194-alist nil)
     (ucs-oriya-to-is13194-alist nil)
     (ucs-tamil-to-is13194-alist nil)
     (ucs-telugu-to-is13194-alist nil)
     (ucs-malayalam-to-is13194-alist nil)
     (ucs-kannada-to-is13194-alist nil))
  (dolist (script '(devanagari bengali assamese gurmukhi gujarati
		    oriya tamil telugu malayalam kannada))
   (let ((hashtable (intern (concat "is13194-to-ucs-"
				    (symbol-name script) "-hashtbl" )))
	 (regexp    (intern (concat "is13194-to-ucs-"
				    (symbol-name script) "-regexp"))))
     (set hashtable (make-hash-table :test 'equal :size 128))
     (dolist (x (eval (intern (concat "ucs-" (symbol-name script)
				      "-to-is13194-alist"))))
       (put-char-code-property (car x) 'script script)
       (put-char-code-property (car x) 'iscii (cdr x))
       (puthash (cdr x) (char-to-string (car x)) (eval hashtable)))
      (set regexp (indian-regexp-of-hashtbl-keys (eval hashtable))))))

(defvar is13194-default-repertory 'devanagari)

(defvar is13194-repertory-to-ucs-script
  `((DEF ?\x40 ,is13194-default-repertory)
    (RMN ?\x41 ,is13194-default-repertory)
    (DEV ?\x42 devanagari)
    (BNG ?\x43 bengali)
    (TML ?\x44 tamil)
    (TLG ?\x45 telugu)
    (ASM ?\x46 bengali)
    (ORI ?\x47 oriya)
    (KND ?\x48 kannada)
    (MLM ?\x49 malayalam)
    (GJR ?\x4a gujarati)
    (PNJ ?\x4b gurmukhi)))

;; for guiding find-variable function.
(defvar is13194-to-ucs-devanagari-hashtbl nil)
(defvar is13194-to-ucs-devanagari-regexp nil)
(defvar is13194-to-ucs-bengali-hashtbl nil)
(defvar is13194-to-ucs-bengali-regexp nil)
(defvar is13194-to-ucs-assamese-hashtbl nil)
(defvar is13194-to-ucs-assamese-regexp nil)
(defvar is13194-to-ucs-gurmukhi-hashtbl nil)
(defvar is13194-to-ucs-gurmukhi-regexp nil)
(defvar is13194-to-ucs-gujarati-hashtbl nil)
(defvar is13194-to-ucs-gujarati-regexp nil)
(defvar is13194-to-ucs-oriya-hashtbl nil)
(defvar is13194-to-ucs-oriya-regexp nil)
(defvar is13194-to-ucs-tamil-hashtbl nil)
(defvar is13194-to-ucs-tamil-regexp nil)
(defvar is13194-to-ucs-telugu-hashtbl nil)
(defvar is13194-to-ucs-telugu-regexp nil)
(defvar is13194-to-ucs-malayalam-hashtbl nil)
(defvar is13194-to-ucs-malayalam-regexp nil)
(defvar is13194-to-ucs-kannada-hashtbl nil)
(defvar is13194-to-ucs-kannada-regexp nil)

(defvar indian-ucs-to-is13194-regexp
  ;; only Devanagari is supported now.
  (concat "[" (char-to-string #x0900)
          "-" (char-to-string #x097f) "]")
  "Regexp that matches to conversion.")

(defun indian-ucs-to-iscii-region (from to)
  "Convert the indian UCS characters in the region to ISCII.
Return new end position."
  (interactive "r")
  ;; only Devanagari is supported now.
  (save-excursion
    (save-restriction
      (narrow-to-region from to)
      (goto-char (point-min))
      ;; (let* ((current-repertory is13194-default-repertory))
	(while (re-search-forward indian-ucs-to-is13194-regexp nil t)
	  (replace-match
	   (get-char-code-property (string-to-char (match-string 0))
				   'iscii)));; )
      (point-max))))

(defun indian-iscii-to-ucs-region (from to)
  "Convert the ISCII characters in the region to UCS.
Return new end position."
  (interactive "r")
  ;; only Devanagari is supported now.
  (save-excursion
    (save-restriction
      (narrow-to-region from to)
      (goto-char (point-min))
      (let* ((current-repertory is13194-default-repertory)
	     (current-hashtable
	      (intern (concat "is13194-to-ucs-"
			      (symbol-name current-repertory) "-hashtbl")))
	     (current-regexp
	      (intern (concat "is13194-to-ucs-"
			      (symbol-name current-repertory) "-regexp")))
	     (re (eval current-regexp))
	     (hash (eval current-hashtable)))
	(while (re-search-forward re nil t)
	  (replace-match (gethash (match-string 0) hash ""))))
      (point-max))))

;;;###autoload
(defun indian-compose-region (from to)
  "Compose the region according to `composition-function-table'."
  (interactive "r")
  (save-excursion
    (save-restriction
      (let ((pos from) newpos func (max to))
	(narrow-to-region from to)
	(while (< pos max)
          ;; FIXME: The below seems to assume
          ;; composition-function-table holds functions?  That is no
          ;; longer true, since long ago.
	  (setq func (aref composition-function-table (char-after pos)))
	  (if (fboundp func)
	      (setq newpos (funcall func pos nil)
		    pos (if (and (integerp newpos) (> newpos pos))
			    newpos (1+ pos)))
	    (setq pos (1+ pos))))))))

;;;###autoload
(defun indian-compose-string (string)
  (with-temp-buffer
    (insert string)
    (indian-compose-region (point-min) (point-max))
    (buffer-string)))

;;;###autoload
(defun in-is13194-post-read-conversion (len)
  (let ((pos (point)) endpos)
    (setq endpos (indian-iscii-to-ucs-region pos (+ pos len)))
    (- endpos pos)))

;;;###autoload
(defun in-is13194-pre-write-conversion (from to)
  (let ((buf (current-buffer)))
    (set-buffer (generate-new-buffer " *temp*"))
    (if (stringp from)
	(insert from)
      (insert-buffer-substring buf from to))
    (indian-ucs-to-iscii-region (point-min) (point-max))
    nil))




;;; Backward Compatibility support programs

;; The following provides the conversion from old-implementation of
;; Emacs Devanagari script to UCS.

(defconst indian-2-colum-to-ucs
  '(
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2120   ö„€€ö„€ö„€‚ö„€ƒö„€„ö„€…ö„€†ö„€‡ö„€ˆö„€‰ö„€Šö„€‹ö„€Œö„€ö„€Ž
  ("ö„€€" . "à¤")
  ("ö„€" . "à¤‚")
  ("ö„€‚" . "à¤ƒ")
  ("ö„€ƒ" . "à¤…")
  ("ö„€„" . "à¤†")
  ("ö„€…" . "à¤‡")
  ("ö„€†" . "à¤ˆ")
  ("ö„€‡" . "à¤‰")
  ("ö„€ˆ" . "à¤Š")
  ("ö„€‰" . "à¤‹")
  ("ö„€‰ö„‚­" . "à¤°à¥ƒ")
  ("ö„€Š" . "à¤Ž")
  ("ö„€‹" . "à¤")
  ("ö„€Œ" . "à¤")
  ("ö„€" . "à¤")
  ("ö„€Ž" . "à¤’")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2130 ö„€ö„€ö„€‘ö„€’ö„€“ö„€”ö„€•ö„€–ö„€—ö„€˜ö„€™ö„€šö„€›ö„€œö„€ö„€ž
  ("ö„€" . "à¤“")
  ("ö„€" . "à¤”")
  ("ö„€‘" . "à¤‘")
  ("ö„€’" . "à¤•")
  ("ö„€“" . "à¤–")
  ("ö„€”" . "à¤—")
  ("ö„€•" . "à¤˜")
  ("ö„€–" . "à¤™")
  ("ö„€—" . "à¤š")
  ("ö„€˜" . "à¤›")
  ("ö„€™" . "à¤œ")
  ("ö„€š" . "à¤")
  ("ö„€›" . "à¤ž")
  ("ö„€œ" . "à¤Ÿ")
  ("ö„€" . "à¤ ")
  ("ö„€ž" . "à¤¡")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2140 ö„€Ÿö„€ ö„€¡ö„€¢ö„€£ö„€¤ö„€¥ö„€¦ö„€§ö„€¨ö„€©ö„€ªö„€«ö„€¬ö„€­ö„€®
  ("ö„€Ÿ" . "à¤¢")
  ("ö„€ " . "à¤£")
  ("ö„€¡" . "à¤¤")
  ("ö„€¢" . "à¤¥")
  ("ö„€£" . "à¤¦")
  ("ö„€¤" . "à¤§")
  ("ö„€¥" . "à¤¨")
  ("ö„€¦" . "à¤©")
  ("ö„€§" . "à¤ª")
  ("ö„€¨" . "à¤«")
  ("ö„€©" . "à¤¬")
  ("ö„€ª" . "à¤­")
  ("ö„€«" . "à¤®")
  ("ö„€¬" . "à¤¯")
  ("ö„€­" . "à¥Ÿ")
  ("ö„€®" . "à¤°")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2150 ö„€¯ö„€°ö„€±ö„€²ö„€³ö„€´ö„€µö„€¶ö„€·ö„€¸ö„€¹ö„€ºö„€»ö„€¼ö„€½ö„€¾
  ("ö„€¯" . "à¤±")
  ("ö„€°" . "à¤²")
  ("ö„€±" . "à¤³")
  ("ö„€²" . "à¤´")
  ("ö„€³" . "à¤µ")
  ("ö„€´" . "à¤¶")
  ("ö„€µ" . "à¤·")
  ("ö„€¶" . "à¤¸")
  ("ö„€·" . "à¤¹")
  ("ö„€¹" . "à¤¾")
  ("ö„€º" . "à¤¿")
  ("ö„€»" . "à¥€")
  ("ö„€¼" . "à¥")
  ("ö„€½" . "à¥‚")
  ("ö„€¾" . "à¥ƒ")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2160 ö„€¿ö„€ö„ö„‚ö„ƒö„„ö„…ö„†ö„‡ö„ˆö„‰ö„Šö„‹ö„Œö„ö„Ž
  ("ö„€¿" . "à¥†")
  ("ö„€" . "à¥‡")
  ("ö„" . "à¥ˆ")
  ("ö„‚" . "à¥…")
  ("ö„ƒ" . "à¥Š")
  ("ö„„" . "à¥‹")
  ("ö„…" . "à¥Œ")
  ("ö„†" . "à¥‰")
  ("ö„‡" . "à¥")
  ("ö„ˆ" . "à¤¼")
  ("ö„‰" . "à¥¤")
  ("ö„‰ö„‰" . "à¥¥")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2170 ö„ö„ö„‘ö„’ö„“ö„”ö„•ö„–ö„—ö„˜ö„™ö„šö„›ö„œö„
  ("ö„" . "à¥¦")
  ("ö„‘" . "à¥§")
  ("ö„’" . "à¥¨")
  ("ö„“" . "à¥©")
  ("ö„”" . "à¥ª")
  ("ö„•" . "à¥«")
  ("ö„–" . "à¥¬")
  ("ö„—" . "à¥­")
  ("ö„˜" . "à¥®")
  ("ö„™" . "à¥¯")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2220   ö„žö„Ÿö„ ö„¡ö„¢ö„£ö„¤ö„¥ö„¦ö„§ö„¨ö„©ö„ªö„«ö„¬
  ("ö„ž" . "à¥›à¥à¤°")
  ("ö„Ÿ" . "à¥žà¥à¤°")
  ("ö„ " . "à¤•à¥à¤°")
  ("ö„¡" . "à¤—à¥à¤°")
  ("ö„¢" . "à¤¤à¥à¤°")
  ("ö„£" . "à¤ªà¥à¤°")
  ("ö„¤" . "à¤«à¥à¤°")
  ("ö„¦" . "à¤¶à¥à¤°")
  ("ö„©" . "à¤°à¥")
  ("ö„ª" . "à¤°à¥‚")
  ("ö„«" . "à¤±à¥")
  ("ö„¬" . "à¤±à¥‚")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2230 ö„­ö„®ö„¯ö„°ö„±ö„²ö„³ö„´ö„µö„¶ö„·ö„¸ö„¹ö„ºö„»ö„¼
  ("ö„°" . "à¤•à¥")
  ("ö„±" . "à¤–à¥")
  ("ö„²" . "à¤—à¥")
  ("ö„³" . "à¤˜à¥")
  ("ö„µ" . "à¤šà¥")
  ("ö„µö„‚®" . "à¤šà¥à¤°à¥")
  ("ö„·" . "à¤œà¥")
  ("ö„¸" . "à¤à¥")
  ("ö„¹" . "à¤žà¥")
  ("ö„¹" . "à¤žà¥")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2240 ö„½ö„¾ö„¿ö„‚€ö„‚ö„‚‚ö„‚ƒö„‚„ö„‚…ö„‚†ö„‚‡ö„‚ˆö„‚‰ö„‚Šö„‚‹ö„‚Œ
  ("ö„¾" . "à¤£à¥")
  ("ö„¿" . "à¤¤à¥")
  ("ö„‚€" . "à¤¥à¥")
  ("ö„‚‚" . "à¤§à¥")
  ("ö„‚ƒ" . "à¤¨à¥")
  ("ö„‚„" . "à¤©à¥")
  ("ö„‚…" . "à¤ªà¥")
  ("ö„‚†" . "à¤«à¥")
  ("ö„‚‡" . "à¤¬à¥")
  ("ö„‚‡" . "à¤¬à¥")
  ("ö„‚ˆ" . "à¤­à¥")
  ("ö„‚‰" . "à¤®à¥")
  ("ö„‚Š" . "à¤¯à¥")
  ("ö„‚‹" . "à¥Ÿà¥")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2250 ö„‚ö„‚Žö„‚ö„‚ö„‚‘ö„‚’ö„‚“ö„‚”ö„‚•ö„‚–ö„‚—ö„‚˜ö„‚™ö„‚šö„‚›ö„‚œ
  ("ö„‚Ž" . "à¤²à¥")
  ("ö„‚" . "à¤³à¥")
  ("ö„‚" . "à¤´à¥")
  ("ö„‚‘" . "à¤µà¥")
  ("ö„‚’" . "à¤¶à¥")
  ("ö„‚“" . "à¤·à¥")
  ("ö„‚”" . "à¤¸à¥")
  ("ö„‚š" . "à¥à¤¯")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2260 ö„‚ö„‚žö„‚Ÿö„‚ ö„‚¡ö„‚¢ö„‚£ö„‚¤ö„‚¥ö„‚¦ö„‚§ö„‚¨ö„‚©ö„‚ªö„‚«ö„‚¬
  ("ö„‚" . "à¤—à¥à¤°à¥")
  ("ö„‚ž" . "à¤˜à¥à¤¨à¥")
  ("ö„‚ " . "à¤¤à¥à¤¤à¥")
  ("ö„‚¡" . "à¤¤à¥à¤°à¥")
  ("ö„‚¢" . "à¤§à¥à¤¨à¥")
  ("ö„‚£" . "à¤§à¥à¤°à¥")
  ("ö„‚¤" . "à¤ªà¥à¤¤à¥")
  ("ö„‚¥" . "à¤¶à¥à¤šà¥")
  ("ö„‚¦" . "à¤¶à¥à¤°à¥")
  ("ö„‚§" . "à¤¶à¥à¤µà¥")
  ("ö„‚¨" . "à¤¨à¥à¤¨à¥")
  ("ö„‚©" . "à¤•à¥à¤·à¥")
  ("ö„‚ª" . "à¤œà¥à¤žà¥")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2270 ö„‚­ö„‚®ö„‚¯ö„‚°ö„‚±ö„‚²ö„‚³ö„‚´ö„‚µö„‚¶ö„‚·ö„‚¸ö„‚¹ö„‚ºö„‚»
  ("ö„‚­" . "à¤°à¥")
  ("ö„‚®" . "à¥à¤°")
  ("ö„‚¯" . "à¥à¤°")
  ("ö„‚°" . "à¥˜à¥")
  ("ö„‚±" . "à¥™à¥")
  ("ö„‚²" . "à¥šà¥")
  ("ö„‚¶" . "à¥žà¥")
  ("ö„‚·" . "à¥›à¥")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2320   ö„‚¼ö„‚½ö„‚¾ö„‚¿ö„ƒ€ö„ƒö„ƒ‚ö„ƒƒö„ƒ„ö„ƒ…ö„ƒ†ö„ƒ‡ö„ƒˆö„ƒ‰ö„ƒŠ
  ("ö„‚¼" . "à¥")
  ("ö„ƒ" . "à¤Œ")
  ("ö„ƒö„‚­" . "à¤°à¥„")
  ("ö„ƒ‚" . "à¥¡")
  ("ö„ƒ‚ö„‚­" . "à¤°à¥£")
  ("ö„ƒ…" . "à¥ ")
  ("ö„ƒ…ö„‚­" . "à¤°à¥¢")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2330 ö„ƒ‹ö„ƒŒö„ƒö„ƒŽö„ƒö„ƒö„ƒ‘ö„ƒ’ö„ƒ“ö„ƒ”ö„ƒ•ö„ƒ–ö„ƒ—ö„ƒ˜ö„ƒ™ö„ƒš
  ("ö„ƒŽ" . "à¥˜")
  ("ö„ƒ" . "à¥™")
  ("ö„ƒ" . "à¥š")
  ("ö„ƒ•" . "à¥›")
  ("ö„ƒš" . "à¥œ")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2340 ö„ƒ›ö„ƒœö„ƒö„ƒžö„ƒŸö„ƒ ö„ƒ¡ö„ƒ¢ö„ƒ£ö„ƒ¤ö„ƒ¥ö„ƒ¦ö„ƒ§ö„ƒ¨ö„ƒ©ö„ƒª
  ("ö„ƒ›" . "à¥")
  ("ö„ƒ¤" . "à¥ž")
  ("ö„ƒ¥" . "à¤½")
  ("ö„ƒ¦" . "à¥„")
  ("ö„ƒ§" . "à¥¢")
  ("ö„ƒ¨" . "à¥£")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2350 ö„ƒ«ö„ƒ¬ö„ƒ­ö„ƒ®ö„ƒ¯ö„ƒ°ö„ƒ±ö„ƒ²ö„ƒ³ö„ƒ´ö„ƒµö„ƒ¶ö„ƒ·ö„ƒ¸ö„ƒ¹ö„ƒº
  ("ö„ƒ«" . "à¤®à¥à¤¨")
  ("ö„ƒ¬" . "à¤®à¥à¤²")
  ("ö„ƒ­" . "à¤¹à¥ƒ")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2360 ö„ƒ»ö„ƒ¼ö„ƒ½ö„ƒ¾ö„ƒ¿ö„„€ö„„ö„„‚ö„„ƒö„„„ö„„…ö„„†ö„„‡ö„„ˆö„„‰ö„„Š
  ("ö„ƒ»" . "à¤²à¥à¤²")
  ("ö„ƒ¼" . "à¤µà¥à¤¨")
  ("ö„ƒ½" . "à¤µà¥à¤µ")
  ("ö„ƒ¾" . "à¤¶à¥à¤š")
  ("ö„ƒ¿" . "à¤¶à¥à¤¨")
  ("ö„„€" . "à¤¶à¥à¤¬")
  ("ö„„" . "à¤¶à¥à¤²")
  ("ö„„‚" . "à¤¶à¥à¤µ")
  ("ö„„ƒ" . "à¤·à¥à¤Ÿà¥à¤°à¥à¤¯")
  ("ö„„„" . "à¤·à¥à¤Ÿà¥à¤¯")
  ("ö„„…" . "à¤·à¥à¤Ÿà¥à¤µ")
  ("ö„„†" . "à¤·à¥à¤Ÿ")
  ("ö„„‡" . "à¤·à¥à¤ ")
  ("ö„„ˆ" . "à¤¸à¥à¤¨")
  ("ö„„‰" . "à¤¸à¥à¤°")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2370 ö„„‹ö„„Œö„„ö„„Žö„„ö„„ö„„‘ö„„’ö„„“ö„„”ö„„•ö„„–ö„„—ö„„˜ö„„™
  ("ö„„‹" . "à¤¹à¥à¤£")
  ("ö„„Œ" . "à¤¹à¥à¤¨")
  ("ö„„" . "à¤¹à¥à¤®")
  ("ö„„Ž" . "à¤¹à¥à¤¯")
  ("ö„„" . "à¤¹à¥à¤°")
  ("ö„„" . "à¤¹à¥à¤²")
  ("ö„„‘" . "à¤¹à¥à¤µ")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2420   ö„„šö„„›ö„„œö„„ö„„žö„„Ÿö„„ ö„„¡ö„„¢ö„„£ö„„¤ö„„¥ö„„¦ö„„§ö„„¨
  ("ö„„š" . "à¤•à¥à¤¤à¥à¤°à¥à¤¯")
  ("ö„„›" . "à¤•à¥à¤¤à¥à¤µ")
  ("ö„„œ" . "à¤•à¥à¤¤à¥à¤¯")
  ("ö„„" . "à¤•à¥à¤¨à¥à¤¯")
  ("ö„„ž" . "à¤•à¥à¤°à¥à¤¯")
  ("ö„„Ÿ" . "à¤•à¥à¤µà¥à¤¯")
  ("ö„„ " . "à¤•à¥à¤•")
  ("ö„„¡" . "à¤•à¥à¤¤")
  ("ö„„¢" . "à¤•à¥à¤¨")
  ("ö„„£" . "à¤•à¥à¤®")
  ("ö„„¤" . "à¤•à¥à¤¯")
  ("ö„„¥" . "à¤•à¥à¤²")
  ("ö„„¦" . "à¤•à¥à¤µ")
  ("ö„„§" . "à¤•à¥à¤·")
  ("ö„„¨" . "à¤˜à¥à¤¨")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2430 ö„„©ö„„ªö„„«ö„„¬ö„„­ö„„®ö„„¯ö„„°ö„„±ö„„²ö„„³ö„„´ö„„µö„„¶ö„„·ö„„¸
  ("ö„„©" . "à¤™à¥à¤•à¥à¤¤à¥à¤¯")
  ("ö„„ª" . "à¤™à¥à¤•à¥à¤·à¥à¤µ")
  ("ö„„«" . "à¤™à¥à¤•à¥à¤¤")
  ("ö„„¬" . "à¤™à¥à¤•à¥à¤·")
  ("ö„„­" . "à¤™à¥à¤˜à¥à¤°")
  ("ö„„®" . "à¤™à¥à¤•à¥à¤¯")
  ("ö„„¯" . "à¤™à¥à¤–à¥à¤¯")
  ("ö„„°" . "à¤™à¥à¤—à¥à¤¯")
  ("ö„„±" . "à¤™à¥à¤˜à¥à¤¯")
  ("ö„„²" . "à¤™à¥à¤•")
  ("ö„„³" . "à¤™à¥à¤–")
  ("ö„„´" . "à¤™à¥à¤—")
  ("ö„„µ" . "à¤™à¥à¤˜")
  ("ö„„¶" . "à¤™à¥à¤™")
  ("ö„„·" . "à¤™à¥à¤¨")
  ("ö„„¸" . "à¤™à¥à¤®")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2440 ö„„¹ö„„ºö„„»ö„„¼ö„„½ö„„¾ö„„¿ö„…€ö„…ö„…‚ö„…ƒö„…„ö„……ö„…†ö„…‡ö„…ˆ
  ("ö„„¹" . "à¤™à¥à¤¯")
  ("ö„„º" . "à¤šà¥à¤š")
  ("ö„„»" . "à¤šà¥à¤ž")
  ("ö„„¼" . "à¤›à¥à¤¯")
  ("ö„„½" . "à¤œà¥à¤°")
  ("ö„„¾" . "à¤œà¥à¤ž")
  ("ö„„¿" . "à¤žà¥à¤š")
  ("ö„…€" . "à¤žà¥à¤œ")
  ("ö„…" . "à¤Ÿà¥à¤•")
  ("ö„…‚" . "à¤Ÿà¥à¤Ÿ")
  ("ö„…ƒ" . "à¤Ÿà¥à¤ ")
  ("ö„…„" . "à¤Ÿà¥à¤¯")
  ("ö„……" . "à¤ à¥à¤¯")
  ("ö„…†" . "à¤¡à¥à¤—à¥à¤¯")
  ("ö„…‡" . "à¤¡à¥à¤˜à¥à¤°")
  ("ö„…ˆ" . "à¤¡à¥à¤°à¥à¤¯")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2450 ö„…‰ö„…Šö„…‹ö„…Œö„…ö„…Žö„…ö„…ö„…‘ö„…’ö„…“ö„…”ö„…•ö„…–ö„…—ö„…˜
  ("ö„…‰" . "à¤¡à¥à¤—")
  ("ö„…Š" . "à¤¡à¥à¤˜")
  ("ö„…‹" . "à¤¡à¥à¤¡")
  ("ö„…Œ" . "à¤¡à¥à¤®")
  ("ö„…" . "à¤¡à¥à¤¯")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2460 ö„…™ö„…šö„…›ö„…œö„…ö„…žö„…Ÿö„… ö„…¡ö„…¢ö„…£ö„…¤ö„…¥ö„…¦ö„…§ö„…¨
  ("ö„…™" . "à¤¢à¥à¤¯")
  ("ö„…š" . "à¤¤à¥à¤¤")
  ("ö„…›" . "à¤¤à¥à¤¨")
  ("ö„…œ" . "à¤¦à¥à¤¦à¥à¤¯")
  ("ö„…" . "à¤¦à¥à¤§à¥à¤¯")
  ("ö„…ž" . "à¤¦à¥à¤­à¥à¤¯")
  ("ö„…Ÿ" . "à¤¦à¥à¤°à¥à¤¯")
  ("ö„… " . "à¤¦à¥à¤µà¥à¤¯")
  ("ö„…¡" . "à¤¦à¥à¤—à¥à¤°")
  ("ö„…¢" . "à¤¦à¥à¤˜à¥à¤°")
  ("ö„…£" . "à¤¦à¥à¤¦à¥à¤µ")
  ("ö„…¤" . "à¤¦à¥à¤§à¥à¤µ")
  ("ö„…¥" . "à¤¦à¥à¤—")
  ("ö„…¦" . "à¤¦à¥à¤˜")
  ("ö„…§" . "à¤¦à¥à¤¦")
  ("ö„…¨" . "à¤¦à¥à¤§")
  ;;      0 1 2 3 4 5 6 7 8 9 a b c d e f
  ;;2470 ö„…©ö„…ªö„…«ö„…¬ö„…­ö„…®ö„…¯ö„…°ö„…±ö„…²ö„…³ö„…´ö„…µö„…¶ö„…·
  ("ö„…©" . "à¤¦à¥à¤¨")
  ("ö„…ª" . "à¤¦à¥à¤¬")
  ("ö„…«" . "à¤¦à¥à¤­")
  ("ö„…¬" . "à¤¦à¥à¤®")
  ("ö„…­" . "à¤¦à¥à¤¯")
  ("ö„…®" . "à¤¦à¥à¤µ")
  ("ö„…¯" . "à¤§à¥à¤¨")
  ("ö„…°" . "à¤¨à¥à¤¨")
  ("ö„…±" . "à¤ªà¥à¤¤")
  ("ö„…²" . "à¤ªà¥à¤¨")
  ("ö„…³" . "à¤ªà¥à¤²")
  ("ö„…´" . "à¤¬à¥à¤¨")
  ("ö„…µ" . "à¤¬à¥à¤¬")
  ("ö„…¶" . "à¤¬à¥à¤µ")
  ("ö„…·" . "à¤­à¥à¤¨")))

(defconst indian-2-column-to-ucs-regexp
  "ö„‰ö„‰\\|ö„µö„‚®\\|[ö„ƒö„ƒ‚ö„€‰ö„ƒ…]ö„‚­\\|[ö„€€-ö„…·]")

(put 'indian-2-column-to-ucs-chartable 'char-table-extra-slots 1)
(defconst indian-2-column-to-ucs-chartable
  (let ((table (make-char-table 'indian-2-column-to-ucs-chartable))
	(alist nil))
    (dolist (elt indian-2-colum-to-ucs)
      (if (= (length (car elt)) 1)
	  (aset table (aref (car elt) 0) (cdr elt))
	(setq alist (cons elt alist))))
    (set-char-table-extra-slot table 0 alist)
    table))

;;;###autoload
(defun indian-2-column-to-ucs-region (from to)
  "Convert old Emacs Devanagari characters to UCS."
  (interactive "r")
  (save-excursion
    (save-restriction
      (let (;; (pos from)
	    (alist (char-table-extra-slot indian-2-column-to-ucs-chartable 0)))
	(narrow-to-region from to)
	(decompose-region from to)
	(goto-char (point-min))
	(while (re-search-forward indian-2-column-to-ucs-regexp nil t)
	  (let ((len (- (match-end 0) (match-beginning 0)))
		subst)
	    (if (= len 1)
		(setq subst (aref indian-2-column-to-ucs-chartable
				  (char-after (match-beginning 0))))
	      (setq subst (cdr (assoc (match-string 0) alist))))
	    (replace-match (if subst subst "?"))))
	(indian-compose-region (point-min) (point-max))))))

(provide 'ind-util)

;;; ind-util.el ends here
