;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; return the elapsed process time in usec
;;; from applying fn to args
;;;
(core:intern (core:find-ns "") "perf" (core:make-ns "perf"))

(core:intern perf "time-delta"
  (:lambda (fn args)
    ((:lambda (start stop)
       (core:difference stop start))
       (core:utime)
       ((:lambda ()
          (core:apply fn args)
          (core:utime))))))

;;;
;;; return an vector of storage consumed
;;; by applying fn to args
;;;
(core:intern perf "storage-delta"
  (:lambda (fn args)
    ((:lambda (start-vec stop-vec)
       (core:fix
        (:lambda (loop)
          (:if (core:eq :vector (core:type-of loop))
               loop
               ((:lambda (list n)
                  ((:lambda (offset)
                     (:if (core:less-than offset (core:vector-len start-vec))
                          ((:lambda (type total alloc in-use)
                             ((:lambda (type-stats)
                                (core:cons `(,@list ,@type-stats) (core:sum 1 n)))
                              `(,type ,total ,alloc ,in-use)))
                           (core:vector-ref start-vec (core:sum offset 0))
                           (core:difference (core:vector-ref stop-vec (core:sum offset 1)) (core:vector-ref start-vec (core:sum offset 1)))
                           (core:difference (core:vector-ref stop-vec (core:sum offset 2)) (core:vector-ref start-vec (core:sum offset 2)))
                           (core:difference (core:vector-ref stop-vec (core:sum offset 3)) (core:vector-ref start-vec (core:sum offset 3))))
                          (core:make-vector :t list)))
                         (core:product n 4)))
                 (core:car loop)
                 (core:cdr loop))))
         (core:cons () 1)))
     (core:heap-stat)
     ((:lambda ()
         (core:apply fn args)
         (core:heap-stat))))))
