;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; list sorting
;;;
(core:intern :prelude "%list-n-of"
   (:lambda (item n)          
     (core:cdr          
      (core:fix
       (:lambda (loop)
         ((:lambda (count list)
            (:if (prelude:zerop count)
                 loop
                 (core:cons (prelude:1- count) (core:cons item list))))
          (core:car loop)
          (core:cdr loop)))
       (core:cons n ())))))

(core:intern :prelude "%max-le"
   (:lambda (root list)
     ((:lambda (next)
        (:if (core:eq next root)
             ()
             `(,@(prelude:%list-n-of next (prelude:count next list)) ,@(prelude:%max-le next list))))
      (prelude:%fx-max-le root list))))

(core:intern :prelude "%min-ge"
   (:lambda (root list)
     ((:lambda (next)
        (:if (core:eq next root)
             ()
             `(,@(prelude:%list-n-of next (prelude:count next list)) ,@(prelude:%min-ge next list))))
      (prelude:%fx-min-ge root list))))

(core:intern :prelude "sort-list-ascending"
   (:lambda (list)
     (:if (prelude:listp list)
          ((:lambda (root max min)
             (:if (core:eq root min)
                  `(,@(prelude:%list-n-of root (prelude:count root list))
                    ,@(prelude:%min-ge root list)
                    ,@(prelude:%max-le root list))
                  (:if (core:eq root max)
                       `(,@(prelude:reverse (prelude:%min-ge root list))
                         ,@(prelude:reverse (prelude:%max-le root list))
                         ,@(prelude:%list-n-of root (prelude:count root list)))
                       `(,@(prelude:reverse (prelude:%max-le root list))
                         ,@(prelude:%list-n-of root (prelude:count root list))
                         ,@(prelude:%min-ge root list)))))
           (core:car list)
           (prelude:%fx-list-max list)
           (prelude:%fx-list-min list))
          (prelude:raise list 'prelude:sort-list-ascending "not a list"))))

(core:intern :prelude "sort-list-descending"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:reverse (prelude:sort-list-ascending list))
           (prelude:raise list 'prelude:sort-list-descending "not a list"))))

;;;
;;; misc
;;;
(core:intern :prelude "%fx-list-max"
   (:lambda (list)
      (prelude:foldl          
       (:lambda (elt acc)
         (:if (core:fx-lt acc elt)
              elt
              acc))
         (core:car list)
         list)))

(core:intern :prelude "%fx-list-min"
   (:lambda (list)
      (prelude:foldl
       (:lambda (elt acc)
         (:if (core:fx-lt acc elt)
              acc
              elt))
         (core:car list)
         list)))

(core:intern :prelude "%fx-max-le"
   (:lambda (max list)
      ((:lambda (min)
          (:if (core:fx-lt max min)
               max
               (prelude:foldl     
                (:lambda (elt acc)
                  (:if (prelude:%andf (core:fx-lt elt max) (core:fx-lt acc elt))
                       elt
                       acc))
                min
                list)))
       (prelude:%fx-list-min list))))

(core:intern :prelude "%fx-min-ge"
   (:lambda (min list)
      ((:lambda (max)
          (:if (core:fx-lt max min)
               min
               (prelude:foldl          
                (:lambda (elt acc)
                  (:if (prelude:%andf (core:fx-lt min elt) (core:fx-lt elt acc))
                       elt
                       acc))
                max
                list)))
         (prelude:%fx-list-max list))))
