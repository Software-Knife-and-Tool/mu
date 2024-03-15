;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; list sorting
;;;
(mu:intern :prelude "%list-n-of"
   (:lambda (item n)          
     (mu:cdr          
      (mu:fix
       (:lambda (loop)
         ((:lambda (count list)
            (:if (prelude:zerop count)
                 loop
                 (mu:cons (prelude:1- count) (mu:cons item list))))
          (mu:car loop)
          (mu:cdr loop)))
       (mu:cons n ())))))

(mu:intern :prelude "%max-le"
   (:lambda (root list)
     ((:lambda (next)
        (:if (mu:eq next root)
             ()
             `(,@(prelude:%list-n-of next (prelude:count next list)) ,@(prelude:%max-le next list))))
      (prelude:%fx-max-le root list))))

(mu:intern :prelude "%min-ge"
   (:lambda (root list)
     ((:lambda (next)
        (:if (mu:eq next root)
             ()
             `(,@(prelude:%list-n-of next (prelude:count next list)) ,@(prelude:%min-ge next list))))
      (prelude:%fx-min-ge root list))))

(mu:intern :prelude "sort-list-ascending"
   (:lambda (list)
     (:if (prelude:listp list)
          ((:lambda (root max min)
             (:if (mu:eq root min)
                  `(,@(prelude:%list-n-of root (prelude:count root list))
                    ,@(prelude:%min-ge root list)
                    ,@(prelude:%max-le root list))
                  (:if (mu:eq root max)
                       `(,@(prelude:reverse (prelude:%min-ge root list))
                         ,@(prelude:reverse (prelude:%max-le root list))
                         ,@(prelude:%list-n-of root (prelude:count root list)))
                       `(,@(prelude:reverse (prelude:%max-le root list))
                         ,@(prelude:%list-n-of root (prelude:count root list))
                         ,@(prelude:%min-ge root list)))))
           (mu:car list)
           (prelude:%fx-list-max list)
           (prelude:%fx-list-min list))
          (prelude:raise list 'prelude:sort-list-ascending "not a list"))))

(mu:intern :prelude "sort-list-descending"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:reverse (prelude:sort-list-ascending list))
           (prelude:raise list 'prelude:sort-list-descending "not a list"))))

;;;
;;; misc
;;;
(mu:intern :prelude "%fx-list-max"
   (:lambda (list)
      (prelude:foldl          
       (:lambda (elt acc)
         (:if (mu:fx-lt acc elt)
              elt
              acc))
         (mu:car list)
         list)))

(mu:intern :prelude "%fx-list-min"
   (:lambda (list)
      (prelude:foldl
       (:lambda (elt acc)
         (:if (mu:fx-lt acc elt)
              acc
              elt))
         (mu:car list)
         list)))

(mu:intern :prelude "%fx-max-le"
   (:lambda (max list)
      ((:lambda (min)
          (:if (mu:fx-lt max min)
               max
               (prelude:foldl     
                (:lambda (elt acc)
                  (:if (prelude:%andf (mu:fx-lt elt max) (mu:fx-lt acc elt))
                       elt
                       acc))
                min
                list)))
       (prelude:%fx-list-min list))))

(mu:intern :prelude "%fx-min-ge"
   (:lambda (min list)
      ((:lambda (max)
          (:if (mu:fx-lt max min)
               min
               (prelude:foldl          
                (:lambda (elt acc)
                  (:if (prelude:%andf (mu:fx-lt min elt) (mu:fx-lt elt acc))
                       elt
                       acc))
                max
                list)))
         (prelude:%fx-list-max list))))
