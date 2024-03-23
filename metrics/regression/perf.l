;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; return the elapsed process time in usec
;;; from applying fn to args
;;;
(core:make-ns :perf)
(core:intern :perf "%tdelta"
  (:lambda (fn args)
    ((:lambda (start stop)
       (core:fx-sub stop start))
       (core:proc-tm)
       ((:lambda ()
          (core:apply fn args)
          (core:proc-tm))))))

;;;
;;; return an vector of storage consumed
;;; by applying fn to args
;;;
(core:intern :perf "%sdelta"
  (:lambda (fn args)
    ((:lambda (start-vec stop-vec)
       (core:fix
        (:lambda (loop)
          (:if (core:eq :vector (core:type-of loop))
               loop
               ((:lambda (list n)
                  ((:lambda (offset)
                     (:if (core:fx-lt offset (core:sv-len start-vec))
                          ((:lambda (type total alloc in-use)
                             ((:lambda (type-stats)
                                (core:cons `(,@list ,@type-stats) (core:fx-add 1 n)))
                              `(,type ,total ,alloc ,in-use)))
                           (core:sv-ref start-vec (core:fx-add offset 0))
                           (core:fx-sub (core:sv-ref stop-vec (core:fx-add offset 1)) (core:sv-ref start-vec (core:fx-add offset 1)))
                           (core:fx-sub (core:sv-ref stop-vec (core:fx-add offset 2)) (core:sv-ref start-vec (core:fx-add offset 2)))
                           (core:fx-sub (core:sv-ref stop-vec (core:fx-add offset 3)) (core:sv-ref start-vec (core:fx-add offset 3))))
                          (core:vector :t list)))
                         (core:fx-mul n 4)))
                 (core:car loop)
                 (core:cdr loop))))
         (core:cons () 1)))
     (core:hp-stat)
     ((:lambda ()
         (core:apply fn args)
         (core:hp-stat))))))
