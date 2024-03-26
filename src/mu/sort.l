;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; list sorting
;;;
(libcore:intern :prelude "%list-n-of"
   (:lambda (item n)          
     (libcore:cdr          
      (libcore:fix
       (:lambda (loop)
         ((:lambda (count list)
            (:if (prelude:zerop count)
                 loop
                 (libcore:cons (prelude:1- count) (libcore:cons item list))))
          (libcore:car loop)
          (libcore:cdr loop)))
       (libcore:cons n ())))))

(libcore:intern :prelude "%max-le"
   (:lambda (root list)
     ((:lambda (next)
        (:if (libcore:eq next root)
             ()
             `(,@(prelude:%list-n-of next (prelude:count next list)) ,@(prelude:%max-le next list))))
      (prelude:%fx-max-le root list))))

(libcore:intern :prelude "%min-ge"
   (:lambda (root list)
     ((:lambda (next)
        (:if (libcore:eq next root)
             ()
             `(,@(prelude:%list-n-of next (prelude:count next list)) ,@(prelude:%min-ge next list))))
      (prelude:%fx-min-ge root list))))

(libcore:intern :prelude "sort-list-ascending"
   (:lambda (list)
     (:if (prelude:listp list)
          ((:lambda (root max min)
             (:if (libcore:eq root min)
                  `(,@(prelude:%list-n-of root (prelude:count root list))
                    ,@(prelude:%min-ge root list)
                    ,@(prelude:%max-le root list))
                  (:if (libcore:eq root max)
                       `(,@(prelude:reverse (prelude:%min-ge root list))
                         ,@(prelude:reverse (prelude:%max-le root list))
                         ,@(prelude:%list-n-of root (prelude:count root list)))
                       `(,@(prelude:reverse (prelude:%max-le root list))
                         ,@(prelude:%list-n-of root (prelude:count root list))
                         ,@(prelude:%min-ge root list)))))
           (libcore:car list)
           (prelude:%fx-list-max list)
           (prelude:%fx-list-min list))
          (prelude:raise list 'prelude:sort-list-ascending "not a list"))))

(libcore:intern :prelude "sort-list-descending"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:reverse (prelude:sort-list-ascending list))
           (prelude:raise list 'prelude:sort-list-descending "not a list"))))

;;;
;;; misc
;;;
(libcore:intern :prelude "%fx-list-max"
   (:lambda (list)
      (prelude:foldl          
       (:lambda (elt acc)
         (:if (libcore:fx-lt acc elt)
              elt
              acc))
         (libcore:car list)
         list)))

(libcore:intern :prelude "%fx-list-min"
   (:lambda (list)
      (prelude:foldl
       (:lambda (elt acc)
         (:if (libcore:fx-lt acc elt)
              acc
              elt))
         (libcore:car list)
         list)))

(libcore:intern :prelude "%fx-max-le"
   (:lambda (max list)
      ((:lambda (min)
          (:if (libcore:fx-lt max min)
               max
               (prelude:foldl     
                (:lambda (elt acc)
                  (:if (prelude:%andf (libcore:fx-lt elt max) (libcore:fx-lt acc elt))
                       elt
                       acc))
                min
                list)))
       (prelude:%fx-list-min list))))

(libcore:intern :prelude "%fx-min-ge"
   (:lambda (min list)
      ((:lambda (max)
          (:if (libcore:fx-lt max min)
               min
               (prelude:foldl          
                (:lambda (elt acc)
                  (:if (prelude:%andf (libcore:fx-lt min elt) (libcore:fx-lt elt acc))
                       elt
                       acc))
                max
                list)))
         (prelude:%fx-list-max list))))
