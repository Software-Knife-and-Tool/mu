;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; list sorting
;;;
(lib:intern :prelude "%list-n-of"
   (:lambda (item n)          
     (lib:cdr          
      (lib:fix
       (:lambda (loop)
         ((:lambda (count list)
            (:if (prelude:zerop count)
                 loop
                 (lib:cons (prelude:1- count) (lib:cons item list))))
          (lib:car loop)
          (lib:cdr loop)))
       (lib:cons n ())))))

(lib:intern :prelude "%max-le"
   (:lambda (root list)
     ((:lambda (next)
        (:if (lib:eq next root)
             ()
             `(,@(prelude:%list-n-of next (prelude:count next list)) ,@(prelude:%max-le next list))))
      (prelude:%fx-max-le root list))))

(lib:intern :prelude "%min-ge"
   (:lambda (root list)
     ((:lambda (next)
        (:if (lib:eq next root)
             ()
             `(,@(prelude:%list-n-of next (prelude:count next list)) ,@(prelude:%min-ge next list))))
      (prelude:%fx-min-ge root list))))

(lib:intern :prelude "sort-list-ascending"
   (:lambda (list)
     (:if (prelude:listp list)
          ((:lambda (root max min)
             (:if (lib:eq root min)
                  `(,@(prelude:%list-n-of root (prelude:count root list))
                    ,@(prelude:%min-ge root list)
                    ,@(prelude:%max-le root list))
                  (:if (lib:eq root max)
                       `(,@(prelude:reverse (prelude:%min-ge root list))
                         ,@(prelude:reverse (prelude:%max-le root list))
                         ,@(prelude:%list-n-of root (prelude:count root list)))
                       `(,@(prelude:reverse (prelude:%max-le root list))
                         ,@(prelude:%list-n-of root (prelude:count root list))
                         ,@(prelude:%min-ge root list)))))
           (lib:car list)
           (prelude:%fx-list-max list)
           (prelude:%fx-list-min list))
          (prelude:raise list 'prelude:sort-list-ascending "not a list"))))

(lib:intern :prelude "sort-list-descending"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:reverse (prelude:sort-list-ascending list))
           (prelude:raise list 'prelude:sort-list-descending "not a list"))))

;;;
;;; misc
;;;
(lib:intern :prelude "%fx-list-max"
   (:lambda (list)
      (prelude:foldl          
       (:lambda (elt acc)
         (:if (lib:fx-lt acc elt)
              elt
              acc))
         (lib:car list)
         list)))

(lib:intern :prelude "%fx-list-min"
   (:lambda (list)
      (prelude:foldl
       (:lambda (elt acc)
         (:if (lib:fx-lt acc elt)
              acc
              elt))
         (lib:car list)
         list)))

(lib:intern :prelude "%fx-max-le"
   (:lambda (max list)
      ((:lambda (min)
          (:if (lib:fx-lt max min)
               max
               (prelude:foldl     
                (:lambda (elt acc)
                  (:if (prelude:%andf (lib:fx-lt elt max) (lib:fx-lt acc elt))
                       elt
                       acc))
                min
                list)))
       (prelude:%fx-list-min list))))

(lib:intern :prelude "%fx-min-ge"
   (:lambda (min list)
      ((:lambda (max)
          (:if (lib:fx-lt max min)
               min
               (prelude:foldl          
                (:lambda (elt acc)
                  (:if (prelude:%andf (lib:fx-lt min elt) (lib:fx-lt elt acc))
                       elt
                       acc))
                max
                list)))
         (prelude:%fx-list-max list))))
