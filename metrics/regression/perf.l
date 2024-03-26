;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; return the elapsed process time in usec
;;; from applying fn to args
;;;
(libcore:make-ns :perf)
(libcore:intern :perf "%tdelta"
  (:lambda (fn args)
    ((:lambda (start stop)
       (libcore:fx-sub stop start))
       (libcore:proc-tm)
       ((:lambda ()
          (libcore:apply fn args)
          (libcore:proc-tm))))))

;;;
;;; return an vector of storage consumed
;;; by applying fn to args
;;;
(libcore:intern :perf "%sdelta"
  (:lambda (fn args)
    ((:lambda (start-vec stop-vec)
       (libcore:fix
        (:lambda (loop)
          (:if (libcore:eq :vector (libcore:type-of loop))
               loop
               ((:lambda (list n)
                  ((:lambda (offset)
                     (:if (libcore:fx-lt offset (libcore:sv-len start-vec))
                          ((:lambda (type total alloc in-use)
                             ((:lambda (type-stats)
                                (libcore:cons `(,@list ,@type-stats) (libcore:fx-add 1 n)))
                              `(,type ,total ,alloc ,in-use)))
                           (libcore:sv-ref start-vec (libcore:fx-add offset 0))
                           (libcore:fx-sub (libcore:sv-ref stop-vec (libcore:fx-add offset 1)) (libcore:sv-ref start-vec (libcore:fx-add offset 1)))
                           (libcore:fx-sub (libcore:sv-ref stop-vec (libcore:fx-add offset 2)) (libcore:sv-ref start-vec (libcore:fx-add offset 2)))
                           (libcore:fx-sub (libcore:sv-ref stop-vec (libcore:fx-add offset 3)) (libcore:sv-ref start-vec (libcore:fx-add offset 3))))
                          (libcore:vector :t list)))
                         (libcore:fx-mul n 4)))
                 (libcore:car loop)
                 (libcore:cdr loop))))
         (libcore:cons () 1)))
     (libcore:hp-stat)
     ((:lambda ()
         (libcore:apply fn args)
         (libcore:hp-stat))))))
